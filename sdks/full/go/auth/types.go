// This file was auto-generated by Fern from our API Definition.

package auth

import (
	fmt "fmt"
)

// Represents the state of an external account linking process.
type CompleteStatus string

const (
	// The linking process succeeded and will now switch identities.
	CompleteStatusSwitchIdentity CompleteStatus = "switch_identity"
	// The linking process succeeded and the new account is now added.
	CompleteStatusLinkedAccountAdded CompleteStatus = "linked_account_added"
	// The current linking process has already completed.
	CompleteStatusAlreadyComplete CompleteStatus = "already_complete"
	// The current linking process has expired.
	CompleteStatusExpired CompleteStatus = "expired"
	// The current linking process has been tried too many times.
	CompleteStatusTooManyAttempts CompleteStatus = "too_many_attempts"
	// The code given to the current linking process is incorrect.
	CompleteStatusIncorrect CompleteStatus = "incorrect"
)

func NewCompleteStatusFromString(s string) (CompleteStatus, error) {
	switch s {
	case "switch_identity":
		return CompleteStatusSwitchIdentity, nil
	case "linked_account_added":
		return CompleteStatusLinkedAccountAdded, nil
	case "already_complete":
		return CompleteStatusAlreadyComplete, nil
	case "expired":
		return CompleteStatusExpired, nil
	case "too_many_attempts":
		return CompleteStatusTooManyAttempts, nil
	case "incorrect":
		return CompleteStatusIncorrect, nil
	}
	var t CompleteStatus
	return "", fmt.Errorf("%s is not a valid %T", s, t)
}

func (c CompleteStatus) Ptr() *CompleteStatus {
	return &c
}