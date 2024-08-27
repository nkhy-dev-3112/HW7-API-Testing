<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>DV</name>
   <tag></tag>
   <elementGuidId>40aaf617-9ed9-40dc-9e31-f92d2d9dff20</elementGuidId>
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
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;: \&quot;${id}\&quot;,\n  \&quot;title\&quot;: \&quot;${title}\&quot;,\n  \&quot;description\&quot;: \&quot;${description}\&quot;,\n  \&quot;attendees\&quot;: ${attendees},\n  \&quot;organizerId\&quot;: \&quot;${organizerId}\&quot;,\n  \&quot;repeated\&quot;: \&quot;${repeated}\&quot;,\n  \&quot;place\&quot;: \&quot;${place}\&quot;,\n  \&quot;groupId\&quot;: \&quot;${groupId}\&quot;,\n  \&quot;timeEnd\&quot;: \&quot;${timeEnd}\&quot;,\n  \&quot;timeStart\&quot;: \&quot;${timeStart}\&quot;\n}&quot;,
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
      <webElementGuid>0f14f475-c918-4224-8b3b-15fd725187cf</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiI4MmVlYTQwMi04ZDU2LTQ0MjctOTIxNC0wNjdhYThjMzFlMmYiLCJyb2xlcyI6WyJTVVBFUl9BRE1JTiJdLCJuYW1lIjoibmtoeTIxQGNsYy5maXR1cy5lZHUudm4iLCJlbWFpbGFkZHJlc3MiOiJua2h5MjFAY2xjLmZpdHVzLmVkdS52biIsIm5hbWVpZGVudGlmaWVyIjoibmtoeTIxQGNsYy5maXR1cy5lZHUudm4iLCJpc3MiOiJNZW50b3JVUy1sb2NhbCIsImlhdCI6MTcyNDMwODcyOCwiZXhwIjoxNzI2OTAwNzI4fQ.2e8Vhq_O6gQejDpWfqJqVvCnke9u1BO_UbMBHZ5Cqartoy8E_713unEAOSVfjBCKyIlKeyWvk-CI3q7nsbVo_Q</value>
      <webElementGuid>c757969d-eb57-4414-b25b-285b774f38df</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://localhost:8080/api/meetings</restUrl>
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
      <id>e0ff715c-3bcb-4649-bf2f-b8f7a0f4c091</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>de2154b1-a74a-4bc4-b296-5c4e36cca93f</id>
      <masked>false</masked>
      <name>title</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>1eb1bf5c-7222-4242-b1d1-729c4e7d11a8</id>
      <masked>false</masked>
      <name>description</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>1f4cbdb4-1c2d-40bf-a326-74f8f9b6f844</id>
      <masked>false</masked>
      <name>attendees</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>f74bf9e9-8b36-421e-aa09-6537830305c4</id>
      <masked>false</masked>
      <name>organizerId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>b6c73bd7-5857-49d7-8bb3-8f453dadc975</id>
      <masked>false</masked>
      <name>repeated</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>b4dd166b-ac78-48ec-b5dc-c235e56577ae</id>
      <masked>false</masked>
      <name>place</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>31ae18a1-25bb-49ea-a462-e541133f0b3d</id>
      <masked>false</masked>
      <name>timeEnd</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>56c9a0c2-c0e5-4327-94dd-22f2d02ed128</id>
      <masked>false</masked>
      <name>timeStart</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>40fa7861-ddd3-4597-b630-bdb48da7a7dc</id>
      <masked>false</masked>
      <name>groupId</name>
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
