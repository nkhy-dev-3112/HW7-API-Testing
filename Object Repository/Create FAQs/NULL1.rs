<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>NULL1</name>
   <tag></tag>
   <elementGuidId>9b6d5420-200c-4c81-a545-87fb0b755fc9</elementGuidId>
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
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;: \&quot;${id}\&quot;\n}\n&quot;,
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
      <webElementGuid>d2d8d965-b01c-4781-8888-903a5b7cb9c7</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiI4MmVlYTQwMi04ZDU2LTQ0MjctOTIxNC0wNjdhYThjMzFlMmYiLCJyb2xlcyI6WyJTVVBFUl9BRE1JTiJdLCJuYW1lIjoibmtoeTIxQGNsYy5maXR1cy5lZHUudm4iLCJlbWFpbGFkZHJlc3MiOiJua2h5MjFAY2xjLmZpdHVzLmVkdS52biIsIm5hbWVpZGVudGlmaWVyIjoibmtoeTIxQGNsYy5maXR1cy5lZHUudm4iLCJpc3MiOiJNZW50b3JVUy1sb2NhbCIsImlhdCI6MTcyNDMwODcyOCwiZXhwIjoxNzI2OTAwNzI4fQ.2e8Vhq_O6gQejDpWfqJqVvCnke9u1BO_UbMBHZ5Cqartoy8E_713unEAOSVfjBCKyIlKeyWvk-CI3q7nsbVo_Q</value>
      <webElementGuid>7f7c7dcb-da7d-4f9b-a9ae-5d937dbac587</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://localhost:8080/api/faqs</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'What is the question?'</defaultValue>
      <description></description>
      <id>b37674eb-8227-4c61-bcfc-31a289436dfb</id>
      <masked>false</masked>
      <name>question</name>
   </variables>
   <variables>
      <defaultValue>'It is an answer'</defaultValue>
      <description></description>
      <id>47531a41-5d81-4270-8f59-5be9998dbbb5</id>
      <masked>false</masked>
      <name>answer</name>
   </variables>
   <variables>
      <defaultValue>'aa8fe403-c230-4b8c-baf8-170b4a6d2ff7'</defaultValue>
      <description></description>
      <id>25a4ea97-3715-485a-aec4-349225c6ffad</id>
      <masked>false</masked>
      <name>groupId</name>
   </variables>
   <variables>
      <defaultValue>'hfkre403-c230-4b8c-baf8-170b4a6d2345'</defaultValue>
      <description></description>
      <id>a0da6172-7079-4aae-b5f9-cff7750ad301</id>
      <masked>false</masked>
      <name>id</name>
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
