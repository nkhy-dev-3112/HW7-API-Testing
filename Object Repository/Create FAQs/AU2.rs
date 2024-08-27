<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AU2</name>
   <tag></tag>
   <elementGuidId>2db6b388-ea15-4d94-a922-f07f634628c8</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiJjZjdmNjdmNy0yZDc4LTQ3YWQtYjQ5My1iMWE3MTBlMDJiYjAiLCJyb2xlcyI6WyJVU0VSIl0sIm5hbWUiOiJua2h5LmRldkBnbWFpbC5jb20iLCJlbWFpbGFkZHJlc3MiOiJua2h5LmRldkBnbWFpbC5jb20iLCJuYW1laWRlbnRpZmllciI6Im5raHkuZGV2QGdtYWlsLmNvbSIsImlzcyI6Ik1lbnRvclVTLWxvY2FsIiwiaWF0IjoxNzI0NDQ1MDc5LCJleHAiOjE3MjcwMzcwNzl9.FgQak8STx4fC-v-Yd2YEZXipEailf-Wb-KFzbBDwx-W0OF6qC3crE9kxjhy1ZTNr1qANvuLWbsgqE7l1TgmKMg</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;question\&quot;: \&quot;${question}\&quot;,\n  \&quot;answer\&quot;: \&quot;${answer}\&quot;,\n  \&quot;groupId\&quot;: \&quot;${groupId}\&quot;,\n  \&quot;id\&quot;: \&quot;${id}\&quot;\n}\n&quot;,
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
      <value>Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiJjZjdmNjdmNy0yZDc4LTQ3YWQtYjQ5My1iMWE3MTBlMDJiYjAiLCJyb2xlcyI6WyJVU0VSIl0sIm5hbWUiOiJua2h5LmRldkBnbWFpbC5jb20iLCJlbWFpbGFkZHJlc3MiOiJua2h5LmRldkBnbWFpbC5jb20iLCJuYW1laWRlbnRpZmllciI6Im5raHkuZGV2QGdtYWlsLmNvbSIsImlzcyI6Ik1lbnRvclVTLWxvY2FsIiwiaWF0IjoxNzI0NDQ1MDc5LCJleHAiOjE3MjcwMzcwNzl9.FgQak8STx4fC-v-Yd2YEZXipEailf-Wb-KFzbBDwx-W0OF6qC3crE9kxjhy1ZTNr1qANvuLWbsgqE7l1TgmKMg</value>
      <webElementGuid>6631f429-394e-48e7-9648-84843c3661b4</webElementGuid>
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
      <defaultValue>''</defaultValue>
      <description></description>
      <id>b37674eb-8227-4c61-bcfc-31a289436dfb</id>
      <masked>false</masked>
      <name>question</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>47531a41-5d81-4270-8f59-5be9998dbbb5</id>
      <masked>false</masked>
      <name>answer</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>25a4ea97-3715-485a-aec4-349225c6ffad</id>
      <masked>false</masked>
      <name>groupId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
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
