<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>DV1</name>
   <tag></tag>
   <elementGuidId>60ad68cb-ea98-415b-8247-cdcbd32c41d6</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiJjZjdmNjdmNy0yZDc4LTQ3YWQtYjQ5My1iMWE3MTBlMDJiYjAiLCJyb2xlcyI6WyJVU0VSIl0sIm5hbWUiOiJua2h5LmRldkBnbWFpbC5jb20iLCJlbWFpbGFkZHJlc3MiOiJua2h5LmRldkBnbWFpbC5jb20iLCJuYW1laWRlbnRpZmllciI6Im5raHkuZGV2QGdtYWlsLmNvbSIsImlzcyI6Ik1lbnRvclVTLWxvY2FsIiwiaWF0IjoxNzI0NDY5ODM2LCJleHAiOjE3MjcwNjE4MzZ9.nFeNVEW1prH071UB3hTpkvmQPiUmUivsy0c7hsnb4L3r8D2UU8km4Aaq9ncvVO_Rf4GFGvSoZE38B4lDJvdFmA</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;channelName\&quot;: \&quot;${channelName}\&quot;,\n  \&quot;description\&quot;: \&quot;${description}\&quot;,\n  \&quot;userIds\&quot;: ${userIds},\n  \&quot;groupId\&quot;: \&quot;${groupId}\&quot;,\n  \&quot;creatorId\&quot;: \&quot;{${creator}\&quot;,\n  \&quot;type\&quot;: \&quot;${type}\&quot;\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>b629d75d-a7b4-497a-b455-bcded656b459</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiJjZjdmNjdmNy0yZDc4LTQ3YWQtYjQ5My1iMWE3MTBlMDJiYjAiLCJyb2xlcyI6WyJVU0VSIl0sIm5hbWUiOiJua2h5LmRldkBnbWFpbC5jb20iLCJlbWFpbGFkZHJlc3MiOiJua2h5LmRldkBnbWFpbC5jb20iLCJuYW1laWRlbnRpZmllciI6Im5raHkuZGV2QGdtYWlsLmNvbSIsImlzcyI6Ik1lbnRvclVTLWxvY2FsIiwiaWF0IjoxNzI0NDY5ODM2LCJleHAiOjE3MjcwNjE4MzZ9.nFeNVEW1prH071UB3hTpkvmQPiUmUivsy0c7hsnb4L3r8D2UU8km4Aaq9ncvVO_Rf4GFGvSoZE38B4lDJvdFmA</value>
      <webElementGuid>978e4a0f-648d-4b76-8910-c3ff0b038b9e</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://localhost:8080/api/channels</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>3c6e54a1-85ca-4d1a-8a8e-3494b552e259</id>
      <masked>false</masked>
      <name>channelName</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>fba57857-1988-4a11-90f0-d4a12bf1676d</id>
      <masked>false</masked>
      <name>description</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>754833c5-1c7d-44de-be6f-3dd511b2ecf2</id>
      <masked>false</masked>
      <name>userIds</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>11ef46ac-cbbf-4642-ae1b-9b9a2a582b70</id>
      <masked>false</masked>
      <name>groupId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>0e1f016c-bcb4-4b54-a554-2cc7c5ea98a8</id>
      <masked>false</masked>
      <name>creatorId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>1c6800c2-0998-4a96-9164-7d1cce91d30e</id>
      <masked>false</masked>
      <name>type</name>
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
