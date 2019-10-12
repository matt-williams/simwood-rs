# account_management_api

All URIs are relative to *https://api.simwood.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
**getAccountType**](account_management_api.md#getAccountType) | **GET** /accounts/{account}/type | Get your current account type, and limitations


# **getAccountType**
> models::AccountTypeResponse getAccountType(ctx, account)
Get your current account type, and limitations

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **account** | **String**|  | 

### Return type

[**models::AccountTypeResponse**](AccountTypeResponse.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

