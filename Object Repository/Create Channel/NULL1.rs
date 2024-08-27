<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>NULL1</name>
   <tag></tag>
   <elementGuidId>12485dfc-9b37-4972-9e1f-69dad0d5bf74</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiI4MmVlYTQwMi04ZDU2LTQ0MjctOTIxNC0wNjdhYThjMzFlMmYiLCJyb2xlcyI6WyJTVVBFUl9BRE1JTiJdLCJuYW1lIjoibmtoeTIxQGNsYy5maXR1cy5lZHUudm4iLCJlbWFpbGFkZHJlc3MiOiJua2h5MjFAY2xjLmZpdHVzLmVkdS52biIsIm5hbWVpZGVudGlmaWVyIjoibmtoeTIxQGNsYy5maXR1cy5lZHUudm4iLCJpc3MiOiJNZW50b3JVUy1sb2NhbCIsImlhdCI6MTcyNDMwODcyOCwiZXhwIjoxNzI2OTAwNzI4fQ.2e8Vhq_O6gQejDpWfqJqVvCnke9u1BO_UbMBHZ5Cqartoy8E_713unEAOSVfjBCKyIlKeyWvk-CI3q7nsbVo_Q</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;description\&quot;: \&quot;${description}\&quot;,\n}\n&quot;,
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
      <value>Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiI4MmVlYTQwMi04ZDU2LTQ0MjctOTIxNC0wNjdhYThjMzFlMmYiLCJyb2xlcyI6WyJTVVBFUl9BRE1JTiJdLCJuYW1lIjoibmtoeTIxQGNsYy5maXR1cy5lZHUudm4iLCJlbWFpbGFkZHJlc3MiOiJua2h5MjFAY2xjLmZpdHVzLmVkdS52biIsIm5hbWVpZGVudGlmaWVyIjoibmtoeTIxQGNsYy5maXR1cy5lZHUudm4iLCJpc3MiOiJNZW50b3JVUy1sb2NhbCIsImlhdCI6MTcyNDMwODcyOCwiZXhwIjoxNzI2OTAwNzI4fQ.2e8Vhq_O6gQejDpWfqJqVvCnke9u1BO_UbMBHZ5Cqartoy8E_713unEAOSVfjBCKyIlKeyWvk-CI3q7nsbVo_Q</value>
      <webElementGuid>8e6ca3f8-9e34-43f2-b259-1bef65ee7233</webElementGuid>
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
