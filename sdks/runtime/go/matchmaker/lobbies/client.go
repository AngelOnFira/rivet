// This file was auto-generated by Fern from our API Definition.

package lobbies

import (
	bytes "bytes"
	context "context"
	json "encoding/json"
	errors "errors"
	fmt "fmt"
	uuid "github.com/google/uuid"
	io "io"
	http "net/http"
	url "net/url"
	sdk "sdk"
	core "sdk/core"
	matchmaker "sdk/matchmaker"
)

type Client struct {
	baseURL string
	caller  *core.Caller
	header  http.Header
}

func NewClient(opts ...core.ClientOption) *Client {
	options := core.NewClientOptions()
	for _, opt := range opts {
		opt(options)
	}
	return &Client{
		baseURL: options.BaseURL,
		caller:  core.NewCaller(options.HTTPClient),
		header:  options.ToHeader(),
	}
}

// Marks the current lobby as ready to accept connections. Players will not be able to connect to this lobby until the lobby is flagged as ready.
// This endpoint requires a [lobby token](/docs/general/concepts/token-types#matchmaker-lobby) for authentication, or a [development namespace token](/docs/general/concepts/token-types#namespace-development) for mock responses. When running on Rivet servers, you can access the given lobby token from the [`RIVET_TOKEN`](/docs/matchmaker/concepts/lobby-env) environment variable.
func (c *Client) Ready(ctx context.Context) error {
	baseURL := "https://api.rivet.gg"
	if c.baseURL != "" {
		baseURL = c.baseURL
	}
	endpointURL := baseURL + "/" + "matchmaker/lobbies/ready"

	errorDecoder := func(statusCode int, body io.Reader) error {
		raw, err := io.ReadAll(body)
		if err != nil {
			return err
		}
		apiError := core.NewAPIError(statusCode, errors.New(string(raw)))
		decoder := json.NewDecoder(bytes.NewReader(raw))
		switch statusCode {
		case 500:
			value := new(sdk.InternalError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 429:
			value := new(sdk.RateLimitError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 403:
			value := new(sdk.ForbiddenError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 408:
			value := new(sdk.UnauthorizedError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 404:
			value := new(sdk.NotFoundError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 400:
			value := new(sdk.BadRequestError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		}
		return apiError
	}

	if err := c.caller.Call(
		ctx,
		&core.CallParams{
			URL:          endpointURL,
			Method:       http.MethodPost,
			Headers:      c.header,
			ErrorDecoder: errorDecoder,
		},
	); err != nil {
		return err
	}
	return nil
}

// If `is_closed` is `true`, the matchmaker will no longer route players to the lobby. Players can still
// join using the /join endpoint (this can be disabled by the developer by rejecting all new connections
// after setting the lobby to closed).
// Does not shutdown the lobby.
//
// This endpoint requires a [lobby token](/docs/general/concepts/token-types#matchmaker-lobby) for
// authentication, or a [development namespace token](/docs/general/concepts/token-types#namespace-development)
// for mock responses. When running on Rivet servers, you can access the given lobby token from the
// [`RIVET_TOKEN`](/docs/matchmaker/concepts/lobby-env) environment variable.
func (c *Client) SetClosed(ctx context.Context, request *matchmaker.SetLobbyClosedRequest) error {
	baseURL := "https://api.rivet.gg"
	if c.baseURL != "" {
		baseURL = c.baseURL
	}
	endpointURL := baseURL + "/" + "matchmaker/lobbies/closed"

	errorDecoder := func(statusCode int, body io.Reader) error {
		raw, err := io.ReadAll(body)
		if err != nil {
			return err
		}
		apiError := core.NewAPIError(statusCode, errors.New(string(raw)))
		decoder := json.NewDecoder(bytes.NewReader(raw))
		switch statusCode {
		case 500:
			value := new(sdk.InternalError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 429:
			value := new(sdk.RateLimitError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 403:
			value := new(sdk.ForbiddenError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 408:
			value := new(sdk.UnauthorizedError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 404:
			value := new(sdk.NotFoundError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 400:
			value := new(sdk.BadRequestError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		}
		return apiError
	}

	if err := c.caller.Call(
		ctx,
		&core.CallParams{
			URL:          endpointURL,
			Method:       http.MethodPut,
			Headers:      c.header,
			Request:      request,
			ErrorDecoder: errorDecoder,
		},
	); err != nil {
		return err
	}
	return nil
}

// Sets the state JSON of the current lobby.
//
// This endpoint requires a [lobby token](/docs/general/concepts/token-types#matchmaker-lobby) for
// authentication, or a [development namespace token](/docs/general/concepts/token-types#namespace-development)
// for mock responses. When running on Rivet servers, you can access the given lobby token from the
// [`RIVET_TOKEN`](/docs/matchmaker/concepts/lobby-env) environment variable.
func (c *Client) SetState(ctx context.Context, request interface{}) error {
	baseURL := "https://api.rivet.gg"
	if c.baseURL != "" {
		baseURL = c.baseURL
	}
	endpointURL := baseURL + "/" + "matchmaker/lobbies/state"

	errorDecoder := func(statusCode int, body io.Reader) error {
		raw, err := io.ReadAll(body)
		if err != nil {
			return err
		}
		apiError := core.NewAPIError(statusCode, errors.New(string(raw)))
		decoder := json.NewDecoder(bytes.NewReader(raw))
		switch statusCode {
		case 500:
			value := new(sdk.InternalError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 429:
			value := new(sdk.RateLimitError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 403:
			value := new(sdk.ForbiddenError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 408:
			value := new(sdk.UnauthorizedError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 404:
			value := new(sdk.NotFoundError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 400:
			value := new(sdk.BadRequestError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		}
		return apiError
	}

	if err := c.caller.Call(
		ctx,
		&core.CallParams{
			URL:          endpointURL,
			Method:       http.MethodPut,
			Headers:      c.header,
			Request:      request,
			ErrorDecoder: errorDecoder,
		},
	); err != nil {
		return err
	}
	return nil
}

// Get the state of any lobby.
//
// This endpoint requires a [lobby token](/docs/general/concepts/token-types#matchmaker-lobby) for
// authentication, or a [development namespace token](/docs/general/concepts/token-types#namespace-development)
// for mock responses. When running on Rivet servers, you can access the given lobby token from the
// [`RIVET_TOKEN`](/docs/matchmaker/concepts/lobby-env) environment variable.
func (c *Client) GetState(ctx context.Context, lobbyId uuid.UUID) (interface{}, error) {
	baseURL := "https://api.rivet.gg"
	if c.baseURL != "" {
		baseURL = c.baseURL
	}
	endpointURL := fmt.Sprintf(baseURL+"/"+"matchmaker/lobbies/%v/state", lobbyId)

	errorDecoder := func(statusCode int, body io.Reader) error {
		raw, err := io.ReadAll(body)
		if err != nil {
			return err
		}
		apiError := core.NewAPIError(statusCode, errors.New(string(raw)))
		decoder := json.NewDecoder(bytes.NewReader(raw))
		switch statusCode {
		case 500:
			value := new(sdk.InternalError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 429:
			value := new(sdk.RateLimitError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 403:
			value := new(sdk.ForbiddenError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 408:
			value := new(sdk.UnauthorizedError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 404:
			value := new(sdk.NotFoundError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 400:
			value := new(sdk.BadRequestError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		}
		return apiError
	}

	var response interface{}
	if err := c.caller.Call(
		ctx,
		&core.CallParams{
			URL:                endpointURL,
			Method:             http.MethodGet,
			Headers:            c.header,
			Response:           &response,
			ResponseIsOptional: true,
			ErrorDecoder:       errorDecoder,
		},
	); err != nil {
		return nil, err
	}
	return response, nil
}

// Finds a lobby based on the given criteria.
// If a lobby is not found and `prevent_auto_create_lobby` is `false`,
// a new lobby will be created.
//
// When [tokenless authentication](/docs/general/concepts/tokenless-authentication/web) is enabled in
// your game namespace, this endpoint does not require a token to authenticate. Otherwise, a
// [development namespace token](/docs/general/concepts/token-types#namespace-development) can be used
// for mock responses and a [public namespace token](/docs/general/concepts/token-types#namespace-public)
// can be used for general authentication.
func (c *Client) Find(ctx context.Context, request *matchmaker.FindLobbyRequest) (*matchmaker.FindLobbyResponse, error) {
	baseURL := "https://api.rivet.gg"
	if c.baseURL != "" {
		baseURL = c.baseURL
	}
	endpointURL := baseURL + "/" + "matchmaker/lobbies/find"

	headers := c.header.Clone()
	if request.Origin != nil {
		headers.Add("origin", fmt.Sprintf("%v", *request.Origin))
	}

	errorDecoder := func(statusCode int, body io.Reader) error {
		raw, err := io.ReadAll(body)
		if err != nil {
			return err
		}
		apiError := core.NewAPIError(statusCode, errors.New(string(raw)))
		decoder := json.NewDecoder(bytes.NewReader(raw))
		switch statusCode {
		case 500:
			value := new(sdk.InternalError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 429:
			value := new(sdk.RateLimitError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 403:
			value := new(sdk.ForbiddenError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 408:
			value := new(sdk.UnauthorizedError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 404:
			value := new(sdk.NotFoundError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 400:
			value := new(sdk.BadRequestError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		}
		return apiError
	}

	var response *matchmaker.FindLobbyResponse
	if err := c.caller.Call(
		ctx,
		&core.CallParams{
			URL:          endpointURL,
			Method:       http.MethodPost,
			Headers:      headers,
			Request:      request,
			Response:     &response,
			ErrorDecoder: errorDecoder,
		},
	); err != nil {
		return nil, err
	}
	return response, nil
}

// Joins a specific lobby.
// This request will use the direct player count configured for the
// lobby group.
//
// When [tokenless authentication](/docs/general/concepts/tokenless-authentication/web) is enabled in
// your game namespace, this endpoint does not require a token to authenticate. Otherwise, a
// [development namespace token](/docs/general/concepts/token-types#namespace-development) can be used
// for mock responses and a [public namespace token](/docs/general/concepts/token-types#namespace-public)
// can be used for general authentication.
func (c *Client) Join(ctx context.Context, request *matchmaker.JoinLobbyRequest) (*matchmaker.JoinLobbyResponse, error) {
	baseURL := "https://api.rivet.gg"
	if c.baseURL != "" {
		baseURL = c.baseURL
	}
	endpointURL := baseURL + "/" + "matchmaker/lobbies/join"

	errorDecoder := func(statusCode int, body io.Reader) error {
		raw, err := io.ReadAll(body)
		if err != nil {
			return err
		}
		apiError := core.NewAPIError(statusCode, errors.New(string(raw)))
		decoder := json.NewDecoder(bytes.NewReader(raw))
		switch statusCode {
		case 500:
			value := new(sdk.InternalError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 429:
			value := new(sdk.RateLimitError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 403:
			value := new(sdk.ForbiddenError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 408:
			value := new(sdk.UnauthorizedError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 404:
			value := new(sdk.NotFoundError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 400:
			value := new(sdk.BadRequestError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		}
		return apiError
	}

	var response *matchmaker.JoinLobbyResponse
	if err := c.caller.Call(
		ctx,
		&core.CallParams{
			URL:          endpointURL,
			Method:       http.MethodPost,
			Headers:      c.header,
			Request:      request,
			Response:     &response,
			ErrorDecoder: errorDecoder,
		},
	); err != nil {
		return nil, err
	}
	return response, nil
}

// Creates a custom lobby.
//
// When [tokenless authentication](/docs/general/concepts/tokenless-authentication/web) is enabled in
// your game namespace, this endpoint does not require a token to authenticate. Otherwise, a
// [development namespace token](/docs/general/concepts/token-types#namespace-development) can be used
// for mock responses and a [public namespace token](/docs/general/concepts/token-types#namespace-public)
// can be used for general authentication.
func (c *Client) Create(ctx context.Context, request *matchmaker.CreateLobbyRequest) (*matchmaker.CreateLobbyResponse, error) {
	baseURL := "https://api.rivet.gg"
	if c.baseURL != "" {
		baseURL = c.baseURL
	}
	endpointURL := baseURL + "/" + "matchmaker/lobbies/create"

	errorDecoder := func(statusCode int, body io.Reader) error {
		raw, err := io.ReadAll(body)
		if err != nil {
			return err
		}
		apiError := core.NewAPIError(statusCode, errors.New(string(raw)))
		decoder := json.NewDecoder(bytes.NewReader(raw))
		switch statusCode {
		case 500:
			value := new(sdk.InternalError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 429:
			value := new(sdk.RateLimitError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 403:
			value := new(sdk.ForbiddenError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 408:
			value := new(sdk.UnauthorizedError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 404:
			value := new(sdk.NotFoundError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 400:
			value := new(sdk.BadRequestError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		}
		return apiError
	}

	var response *matchmaker.CreateLobbyResponse
	if err := c.caller.Call(
		ctx,
		&core.CallParams{
			URL:          endpointURL,
			Method:       http.MethodPost,
			Headers:      c.header,
			Request:      request,
			Response:     &response,
			ErrorDecoder: errorDecoder,
		},
	); err != nil {
		return nil, err
	}
	return response, nil
}

// Lists all open lobbies.
//
// When [tokenless authentication](/docs/general/concepts/tokenless-authentication/web) is enabled in
// your game namespace, this endpoint does not require a token to authenticate. Otherwise, a
// [development namespace token](/docs/general/concepts/token-types#namespace-development) can be used
// for mock responses and a [public namespace token](/docs/general/concepts/token-types#namespace-public)
// can be used for general authentication.
func (c *Client) List(ctx context.Context, request *matchmaker.ListLobbiesRequest) (*matchmaker.ListLobbiesResponse, error) {
	baseURL := "https://api.rivet.gg"
	if c.baseURL != "" {
		baseURL = c.baseURL
	}
	endpointURL := baseURL + "/" + "matchmaker/lobbies/list"

	queryParams := make(url.Values)
	if request.IncludeState != nil {
		queryParams.Add("include_state", fmt.Sprintf("%v", *request.IncludeState))
	}
	if len(queryParams) > 0 {
		endpointURL += "?" + queryParams.Encode()
	}

	errorDecoder := func(statusCode int, body io.Reader) error {
		raw, err := io.ReadAll(body)
		if err != nil {
			return err
		}
		apiError := core.NewAPIError(statusCode, errors.New(string(raw)))
		decoder := json.NewDecoder(bytes.NewReader(raw))
		switch statusCode {
		case 500:
			value := new(sdk.InternalError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 429:
			value := new(sdk.RateLimitError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 403:
			value := new(sdk.ForbiddenError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 408:
			value := new(sdk.UnauthorizedError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 404:
			value := new(sdk.NotFoundError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 400:
			value := new(sdk.BadRequestError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		}
		return apiError
	}

	var response *matchmaker.ListLobbiesResponse
	if err := c.caller.Call(
		ctx,
		&core.CallParams{
			URL:          endpointURL,
			Method:       http.MethodGet,
			Headers:      c.header,
			Response:     &response,
			ErrorDecoder: errorDecoder,
		},
	); err != nil {
		return nil, err
	}
	return response, nil
}