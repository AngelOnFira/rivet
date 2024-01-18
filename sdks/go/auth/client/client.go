// This file was auto-generated by Fern from our API Definition.

package client

import (
	http "net/http"
	identityclient "sdk/auth/identity/client"
	tokens "sdk/auth/tokens"
	core "sdk/core"
)

type Client struct {
	baseURL string
	caller  *core.Caller
	header  http.Header

	Identity *identityclient.Client
	Tokens   *tokens.Client
}

func NewClient(opts ...core.ClientOption) *Client {
	options := core.NewClientOptions()
	for _, opt := range opts {
		opt(options)
	}
	return &Client{
		baseURL:  options.BaseURL,
		caller:   core.NewCaller(options.HTTPClient),
		header:   options.ToHeader(),
		Identity: identityclient.NewClient(opts...),
		Tokens:   tokens.NewClient(opts...),
	}
}
