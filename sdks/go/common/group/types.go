// This file was auto-generated by Fern from our API Definition.

package group

import (
	json "encoding/json"
	fmt "fmt"
	uuid "github.com/google/uuid"
	sdk "sdk"
	core "sdk/core"
)

// External links for this group.
type ExternalLinks struct {
	// A link to this group's profile page.
	Profile string `json:"profile"`

	_rawJSON json.RawMessage
}

func (e *ExternalLinks) UnmarshalJSON(data []byte) error {
	type unmarshaler ExternalLinks
	var value unmarshaler
	if err := json.Unmarshal(data, &value); err != nil {
		return err
	}
	*e = ExternalLinks(value)
	e._rawJSON = json.RawMessage(data)
	return nil
}

func (e *ExternalLinks) String() string {
	if len(e._rawJSON) > 0 {
		if value, err := core.StringifyJSON(e._rawJSON); err == nil {
			return value
		}
	}
	if value, err := core.StringifyJSON(e); err == nil {
		return value
	}
	return fmt.Sprintf("%#v", e)
}

// A group handle.
type Handle struct {
	GroupId     uuid.UUID       `json:"group_id"`
	DisplayName sdk.DisplayName `json:"display_name"`
	// The URL of this group's avatar image
	AvatarUrl *string        `json:"avatar_url,omitempty"`
	External  *ExternalLinks `json:"external,omitempty"`
	// Whether or not this group is a developer group.
	IsDeveloper *bool `json:"is_developer,omitempty"`

	_rawJSON json.RawMessage
}

func (h *Handle) UnmarshalJSON(data []byte) error {
	type unmarshaler Handle
	var value unmarshaler
	if err := json.Unmarshal(data, &value); err != nil {
		return err
	}
	*h = Handle(value)
	h._rawJSON = json.RawMessage(data)
	return nil
}

func (h *Handle) String() string {
	if len(h._rawJSON) > 0 {
		if value, err := core.StringifyJSON(h._rawJSON); err == nil {
			return value
		}
	}
	if value, err := core.StringifyJSON(h); err == nil {
		return value
	}
	return fmt.Sprintf("%#v", h)
}

// The current publicity value for the given group.
type Publicity string

const (
	PublicityOpen   Publicity = "open"
	PublicityClosed Publicity = "closed"
)

func NewPublicityFromString(s string) (Publicity, error) {
	switch s {
	case "open":
		return PublicityOpen, nil
	case "closed":
		return PublicityClosed, nil
	}
	var t Publicity
	return "", fmt.Errorf("%s is not a valid %T", s, t)
}

func (p Publicity) Ptr() *Publicity {
	return &p
}

type Summary struct {
	GroupId     uuid.UUID       `json:"group_id"`
	DisplayName sdk.DisplayName `json:"display_name"`
	// The URL of this group's avatar image.
	AvatarUrl *string        `json:"avatar_url,omitempty"`
	External  *ExternalLinks `json:"external,omitempty"`
	// Whether or not this group is a developer.
	IsDeveloper bool    `json:"is_developer"`
	Bio         sdk.Bio `json:"bio"`
	// Whether or not the current identity is a member of this group.
	IsCurrentIdentityMember bool      `json:"is_current_identity_member"`
	Publicity               Publicity `json:"publicity,omitempty"`
	MemberCount             int       `json:"member_count"`
	OwnerIdentityId         uuid.UUID `json:"owner_identity_id"`

	_rawJSON json.RawMessage
}

func (s *Summary) UnmarshalJSON(data []byte) error {
	type unmarshaler Summary
	var value unmarshaler
	if err := json.Unmarshal(data, &value); err != nil {
		return err
	}
	*s = Summary(value)
	s._rawJSON = json.RawMessage(data)
	return nil
}

func (s *Summary) String() string {
	if len(s._rawJSON) > 0 {
		if value, err := core.StringifyJSON(s._rawJSON); err == nil {
			return value
		}
	}
	if value, err := core.StringifyJSON(s); err == nil {
		return value
	}
	return fmt.Sprintf("%#v", s)
}