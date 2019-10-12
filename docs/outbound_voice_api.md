# outbound_voice_api

All URIs are relative to *https://api.simwood.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
**deleteOutboundAclIp**](outbound_voice_api.md#deleteOutboundAclIp) | **DELETE** /voice/{account}/outbound/{trunk}/acl/{ip} | Remove IP from ACL-based trunk
**deleteOutboundTrunk**](outbound_voice_api.md#deleteOutboundTrunk) | **DELETE** /voice/{account}/outbound/{trunk} | Delete the trunk
**getOutboundAclIps**](outbound_voice_api.md#getOutboundAclIps) | **GET** /voice/{account}/outbound/{trunk}/acl | Request information of specified trunk
**getOutboundTrunk**](outbound_voice_api.md#getOutboundTrunk) | **GET** /voice/{account}/outbound/{trunk} | Request information of specified trunk
**getOutboundTrunks**](outbound_voice_api.md#getOutboundTrunks) | **GET** /voice/{account}/outbound | List all active outbound trunks
**putOutboundAclIp**](outbound_voice_api.md#putOutboundAclIp) | **PUT** /voice/{account}/outbound/{trunk}/acl/{ip} | Add IP to ACL-based trunk
**putOutboundTrunk**](outbound_voice_api.md#putOutboundTrunk) | **PUT** /voice/{account}/outbound/{trunk} | Create new trunk or update existing trunk


# **deleteOutboundAclIp**
> deleteOutboundAclIp(ctx, account, trunk, ip)
Remove IP from ACL-based trunk

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **account** | **String**|  | 
  **trunk** | **String**|  | 
  **ip** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteOutboundTrunk**
> deleteOutboundTrunk(ctx, account, trunk)
Delete the trunk

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **account** | **String**|  | 
  **trunk** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getOutboundAclIps**
> models::OutboundAclIps getOutboundAclIps(ctx, account, trunk)
Request information of specified trunk

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **account** | **String**|  | 
  **trunk** | **String**|  | 

### Return type

[**models::OutboundAclIps**](OutboundAclIps.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getOutboundTrunk**
> models::OutboundTrunk getOutboundTrunk(ctx, account, trunk)
Request information of specified trunk

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **account** | **String**|  | 
  **trunk** | **String**|  | 

### Return type

[**models::OutboundTrunk**](OutboundTrunk.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getOutboundTrunks**
> models::OutboundTrunksResponse getOutboundTrunks(ctx, account)
List all active outbound trunks

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **account** | **String**|  | 

### Return type

[**models::OutboundTrunksResponse**](OutboundTrunksResponse.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **putOutboundAclIp**
> putOutboundAclIp(ctx, account, trunk, ip)
Add IP to ACL-based trunk

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **account** | **String**|  | 
  **trunk** | **String**|  | 
  **ip** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **putOutboundTrunk**
> models::OutboundTrunk putOutboundTrunk(ctx, account, trunk, optional)
Create new trunk or update existing trunk

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **account** | **String**|  | 
  **trunk** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **account** | **String**|  | 
 **trunk** | **String**|  | 
 **outbound_trunk** | [**OutboundTrunk**](OutboundTrunk.md)| Trunk to create or update | 

### Return type

[**models::OutboundTrunk**](OutboundTrunk.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

