// This file was auto-generated by Fern from our API Definition.

package identity

import (
	json "encoding/json"
	fmt "fmt"
	sdk "sdk"
	core "sdk/core"
)

type CompleteAccessTokenVerificationRequest struct {
	AccessToken sdk.Jwt `json:"access_token"`

	_rawJSON json.RawMessage
}

func (c *CompleteAccessTokenVerificationRequest) UnmarshalJSON(data []byte) error {
	type unmarshaler CompleteAccessTokenVerificationRequest
	var value unmarshaler
	if err := json.Unmarshal(data, &value); err != nil {
		return err
	}
	*c = CompleteAccessTokenVerificationRequest(value)
	c._rawJSON = json.RawMessage(data)
	return nil
}

func (c *CompleteAccessTokenVerificationRequest) String() string {
	if len(c._rawJSON) > 0 {
		if value, err := core.StringifyJSON(c._rawJSON); err == nil {
			return value
		}
	}
	if value, err := core.StringifyJSON(c); err == nil {
		return value
	}
	return fmt.Sprintf("%#v", c)
}
