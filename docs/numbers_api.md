# numbers_api

All URIs are relative to *https://api.simwood.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
**deleteAllocatedNumber**](numbers_api.md#deleteAllocatedNumber) | **DELETE** /numbers/{account}/allocated/{number} | De-configure and irrevocably remove number from account
**deleteNumberConfig**](numbers_api.md#deleteNumberConfig) | **DELETE** /numbers/{account}/allocated/{number}/config | De-configure the configuration of number
**getAllocatedNumber**](numbers_api.md#getAllocatedNumber) | **GET** /numbers/{account}/allocated/{number} | Return configuration information on allocated number
**getAllocatedNumbers**](numbers_api.md#getAllocatedNumbers) | **POST** /numbers/{account}/allocated/all | Request a report of all current allocated numbers on account
**getAvailableNumbers**](numbers_api.md#getAvailableNumbers) | **GET** /numbers/{account}/available/{tier}/{number} | Returns 1,10 or 100 numbers available for allocation matching the pattern specified
**getNumberConfig**](numbers_api.md#getNumberConfig) | **GET** /numbers/{account}/allocated/{number}/config | Return configuration information on allocated number
**getNumberRanges**](numbers_api.md#getNumberRanges) | **GET** /numbers/{account}/ranges | Retrieves a list of all available number ranges, including descriptions
**putAllocatedNumber**](numbers_api.md#putAllocatedNumber) | **PUT** /numbers/{account}/allocated/{number} | Allocate an available number to the account
**putNumberConfig**](numbers_api.md#putNumberConfig) | **PUT** /numbers/{account}/allocated/{number}/config | Replace active configuration for number


# **deleteAllocatedNumber**
> deleteAllocatedNumber(ctx, account, number)
De-configure and irrevocably remove number from account

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **account** | **String**|  | 
  **number** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteNumberConfig**
> deleteNumberConfig(ctx, account, number)
De-configure the configuration of number

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **account** | **String**|  | 
  **number** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getAllocatedNumber**
> models::AllocatedNumberResponse getAllocatedNumber(ctx, account, number)
Return configuration information on allocated number

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **account** | **String**|  | 
  **number** | **String**|  | 

### Return type

[**models::AllocatedNumberResponse**](AllocatedNumberResponse.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getAllocatedNumbers**
> models::AllocatedNumbersResponse getAllocatedNumbers(ctx, account)
Request a report of all current allocated numbers on account

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **account** | **String**|  | 

### Return type

[**models::AllocatedNumbersResponse**](AllocatedNumbersResponse.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getAvailableNumbers**
> models::AvailableNumbersResponse getAvailableNumbers(ctx, account, tier, number, optional)
Returns 1,10 or 100 numbers available for allocation matching the pattern specified

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **account** | **String**|  | 
  **tier** | **String**|  | 
  **number** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **account** | **String**|  | 
 **tier** | **String**|  | 
 **number** | **i32**|  | 
 **pattern** | **String**|  | 

### Return type

[**models::AvailableNumbersResponse**](AvailableNumbersResponse.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getNumberConfig**
> models::NumberConfig getNumberConfig(ctx, account, number)
Return configuration information on allocated number

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **account** | **String**|  | 
  **number** | **String**|  | 

### Return type

[**models::NumberConfig**](NumberConfig.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getNumberRanges**
> models::NumberRangesResponse getNumberRanges(ctx, account)
Retrieves a list of all available number ranges, including descriptions

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **account** | **String**|  | 

### Return type

[**models::NumberRangesResponse**](NumberRangesResponse.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **putAllocatedNumber**
> putAllocatedNumber(ctx, account, number)
Allocate an available number to the account

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **account** | **String**|  | 
  **number** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **putNumberConfig**
> models::PutNumberConfigResponse putNumberConfig(ctx, account, number, optional)
Replace active configuration for number

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **account** | **String**|  | 
  **number** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **account** | **String**|  | 
 **number** | **String**|  | 
 **number_config** | [**NumberConfig**](NumberConfig.md)| New number configuration | 

### Return type

[**models::PutNumberConfigResponse**](PutNumberConfigResponse.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

