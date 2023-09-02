{% if grains['volumes']['ats']['mount'] %}
{% set device = '/dev/disk/by-id/scsi-0Linode_Volume_' ~ grains['rivet']['name'] ~ '-ats' %}
disk_create_traffic_serer:
  blockdev.formatted:
    - name: {{ device }}
    - fs_type: ext4

disk_mount_traffic_server:
  file.directory:
    - name: /mnt/trafficserver
    - makedirs: True
  mount.mounted:
    - name: /mnt/trafficserver
    - device: {{ device }}
    - fstype: ext4
    - require:
      - blockdev: disk_create_traffic_serer
{% endif %}

create_trafficserver_user:
  user.present:
    - name: trafficserver
    - shell: /bin/false
    - system: True
    - usergroup: True

create_mnt_db_trafficserver:
  file.directory:
    - names:
      - /mnt/trafficserver/db
      - /var/log/trafficserver
      - /run/trafficserver
    - user: trafficserver
    - group: trafficserver
    - mode: 700
    - makedirs: True
    - require:
      - user: create_trafficserver_user
      {%- if grains['volumes']['ats']['mount'] %}
      - mount: disk_mount_traffic_server
      {%- endif %}

push_trafficserver_service:
  file.managed:
    - name: /etc/systemd/system/trafficserver.service
    - source: salt://traffic_server/files/trafficserver.service
    - template: jinja
    - onchanges:
      - cmd: build_nix_shell

start_trafficserver_service:
  cmd.run:
    - name: systemctl daemon-reload && systemctl restart trafficserver
    - require:
      - file: create_mnt_db_trafficserver
      - file: push_trafficserver_service
    - onchanges:
      - file: push_trafficserver_service

push_etc_trafficserver_static:
  file.recurse:
    - name: /etc/trafficserver/
    - source: salt://traffic_server/files/etc/static/
    - user: trafficserver
    - group: trafficserver
    - file_mode: 644
    - dir_mode: 755
    # Keep other files, since we'll also be writing files in push_etc_trafficserver_dynamic
    - clean: False
    - require:
      - user: create_trafficserver_user

push_etc_trafficserver_dynamic:
  file.managed:
    - names:
      - /etc/trafficserver/records.config:
        - source: salt://traffic_server/files/etc/dynamic/records.config.j2
      - /etc/trafficserver/remap.config:
        - source: salt://traffic_server/files/etc/dynamic/remap.config.j2
      - /etc/trafficserver/storage.config:
        - source: salt://traffic_server/files/etc/dynamic/storage.config.j2
    - user: trafficserver
    - group: trafficserver
    - mode: 644
    - template: jinja
    - context:
        nebula_ipv4: {{ grains['nebula']['ipv4'] }}
        s3_providers: {{ pillar['s3']['config'] }}
        volume_size_cache: {{ grains['volumes']['ats']['size']|int - 1 }}G
    - require:
      - file: push_etc_trafficserver_static
      - user: create_trafficserver_user

{%- for provider, _ in pillar['s3']['config'].items() %}
push_etc_trafficserver_dynamic_{{provider}}:
  file.managed:
    - names:
      - /etc/trafficserver/s3_auth_v4_{{provider}}.config:
        - source: salt://traffic_server/files/etc/dynamic/s3_auth_v4.config.j2
      - /etc/trafficserver/s3_region_map_{{provider}}.config:
        - source: salt://traffic_server/files/etc/dynamic/s3_region_map.config.j2
    - user: trafficserver
    - group: trafficserver
    - mode: 644
    - template: jinja
    - context:
        s3_endpoint: {{ pillar['s3']['config'][provider]['endpoint_internal'] }}
        s3_region: {{ pillar['s3']['config'][provider]['region'] }}
        s3_access_key_id: {{ pillar['s3']['access'][provider]['persistent_access_key_id'] }}
        s3_secret_access_key: {{ pillar['s3']['access'][provider]['persistent_access_key_secret'] }}
        s3_region_map_file_name: s3_region_map_{{provider}}
    - require:
      - file: push_etc_trafficserver_static
      - user: create_trafficserver_user
{%- endfor %}

reload_traffic_server_config:
  cmd.run:
    - name: /var/rivet-nix/result/traffic_server/bin/traffic_ctl config reload
    - require:
      - cmd: start_trafficserver_service
      - file: push_etc_trafficserver_static
      - file: push_etc_trafficserver_dynamic
      {%- for provider, _ in pillar['s3']['config'].items() %}
      - push_etc_trafficserver_dynamic_{{provider}}
      {%- endfor %}
    - onchanges:
      - file: push_etc_trafficserver_static
      - file: push_etc_trafficserver_dynamic
      {%- for provider, _ in pillar['s3']['config'].items() %}
      - push_etc_trafficserver_dynamic_{{provider}}
      {%- endfor %}

push_etc_consul_traffic_server_hcl:
  file.managed:
    - name: /etc/consul.d/traffic_server.hcl
    - source: salt://traffic_server/files/consul/traffic_server.hcl.j2
    - template: jinja
    - context:
        namespace: {{ pillar['rivet']['namespace'] }}
        domain: {{ pillar['rivet']['domain'] }}
        nebula_ipv4: {{ grains['nebula']['ipv4'] }}
        s3_providers: {{ pillar['s3']['config'] }}
    - require:
      - file: create_etc_consul
  cmd.run:
    - name: consul reload
    - require:
      - service: start_consul_service
    - onchanges:
      - file: push_etc_consul_traffic_server_hcl

