# \ModuleApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**module_call**](ModuleApi.md#module_call) | **POST** /modules/{module}/functions/{func}/call | 



## module_call

> crate::models::ModuleCallResponse module_call(module, func, module_call_request, origin)


Makes a request to a module's function. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**module** | **String** |  | [required] |
**func** | **String** |  | [required] |
**module_call_request** | [**ModuleCallRequest**](ModuleCallRequest.md) |  | [required] |
**origin** | Option<**String**> |  |  |

### Return type

[**crate::models::ModuleCallResponse**](ModuleCallResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
