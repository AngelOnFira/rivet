{% import_json "/srv/salt-context/rivet/secrets.json" as rivet_secrets %}

ats: {{ rivet_secrets['ats'] }}
