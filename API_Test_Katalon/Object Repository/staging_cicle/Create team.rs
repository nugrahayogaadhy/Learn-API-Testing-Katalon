<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create team</name>
   <tag></tag>
   <elementGuidId>d616fe8b-0d36-47fd-aae1-60d16048cdef</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;name\&quot;: \&quot;postman 16\&quot;,\n    \&quot;desc\&quot;: \&quot;postman baru 16 desc\&quot;,\n    \&quot;type\&quot;: \&quot;project\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>jwt eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjp7Il9pZCI6IjYzNDZjNTZmNTc1NzdhMWM5NDhhODcwMiIsImdvb2dsZUlkIjoiMTEyMTY4NzU5MDg4NzU3MDEwOTQ4IiwiZW1haWwiOiJudWdyYWhheW9nYWFkaHlAZ21haWwuY29tIiwiZnVsbE5hbWUiOiJZb2dhIEFkaHkgTnVncmFoYSIsInBob3RvVXJsIjoiaHR0cHM6Ly9saDMuZ29vZ2xldXNlcmNvbnRlbnQuY29tL2EvQUxtNXd1ME52R1JxV3VNVmlKMzNrZEJlS1lUX2FmTjV3dGs4V1hyQkxLOHM9czk2LWMiLCJiaW8iOiIiLCJzdGF0dXMiOiIiLCJjcmVhdGVkQXQiOiIyMDIyLTEwLTEyVDEzOjQ3OjI3LjM3M1oiLCJ1cGRhdGVkQXQiOiIyMDIyLTExLTE4VDE0OjQwOjU1LjQ1MVoiLCJfX3YiOjAsImRlZmF1bHRDb21wYW55Ijp7Il9pZCI6IjYzNzc5OTc3MGEwMmFhYjQ3OTFiMTExOCJ9fSwiaWF0IjoxNjY5MTczNjI0LCJleHAiOjE2NzE3NjU2MjR9.2KTbjAapVe0dOhu-fhxI6y-FMJRYErpUcQ7VhyF3W20</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <katalonVersion>7.5.5</katalonVersion>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://stagingapi.cicle.app/api/v1/teams?companyId=637799770a02aab4791b1118</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.baseURL_Cicle</defaultValue>
      <description></description>
      <id>b4ceab24-1b9f-4284-8fca-55c135937936</id>
      <masked>false</masked>
      <name>baseURL</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
