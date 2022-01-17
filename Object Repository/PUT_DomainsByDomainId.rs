<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PUT_DomainsByDomainId</name>
   <tag></tag>
   <elementGuidId>a0294030-60d0-46f4-9969-d24281973dfd</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;name\&quot;: \&quot;${domain}\&quot;,\n    \&quot;isInternal\&quot;: ${isinternal},\n    \&quot;configurations\&quot;: [\n        {\n            \&quot;host\&quot;: \&quot;${host}\&quot;,\n            \&quot;port\&quot;: ${port},\n            \&quot;ssl\&quot;: ${ssl},\n            \&quot;type\&quot;: \&quot;${protocol}\&quot;\n        } \n    ]\n} &quot;,
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
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${API_URL}/domains/${domainId}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>361096d8-446c-4e7f-ad8c-dc06324e1c18</id>
      <masked>false</masked>
      <name>domainId</name>
   </variables>
   <variables>
      <defaultValue>'http://www.mocky.io/v2/5db260c4350000e117f54f7e'</defaultValue>
      <description></description>
      <id>583f0101-a3dc-4105-8188-5e963add6e61</id>
      <masked>false</masked>
      <name>API_URL</name>
   </variables>
   <variables>
      <defaultValue>true</defaultValue>
      <description></description>
      <id>98ab92a2-aa8d-4434-b43b-f00d584721c8</id>
      <masked>false</masked>
      <name>isinternal</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>77d61f4b-2eed-4144-a929-6eb3cea5a6ee</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>3154c972-b34e-414f-8f73-059a733bf49a</id>
      <masked>false</masked>
      <name>port</name>
   </variables>
   <variables>
      <defaultValue>true</defaultValue>
      <description></description>
      <id>bf1aa081-adc8-4b54-b914-8e7f257f1109</id>
      <masked>false</masked>
      <name>ssl</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>258c2daa-50cf-4210-9244-9b9bd06d9da2</id>
      <masked>false</masked>
      <name>protocol</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
