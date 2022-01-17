<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>requestbin</name>
   <tag></tag>
   <elementGuidId>932dd7d7-de70-446e-a47d-75a48510abd1</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;multipart/form-data&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;abc&quot;,
      &quot;value&quot;: &quot;123&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;def&quot;,
      &quot;value&quot;: &quot;456&quot;,
      &quot;type&quot;: &quot;Text&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>multipart/form-data</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>UUID</name>
      <type>Main</type>
      <value>${UUID.randomUUID().toString()} </value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${baseUrl}/?a=${dog}&amp;b=con mèo&amp;c=con chuột&amp;d=con heo&amp;e=${test}&amp;f=~!@$*():/,+'&amp;g=</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'wine'</defaultValue>
      <description></description>
      <id>d13e1af7-1406-4d22-a93e-da9ad8c7270a</id>
      <masked>false</masked>
      <name>productName</name>
   </variables>
   <variables>
      <defaultValue>'`~!@#$%^&amp;*()_+-=[]{}|;’:”,./&lt;>?'</defaultValue>
      <description></description>
      <id>44c77cbb-66a0-4983-8611-796920a6df39</id>
      <masked>false</masked>
      <name>test</name>
   </variables>
   <variables>
      <defaultValue>'https://enknuhk10969q.x.pipedream.net/'</defaultValue>
      <description></description>
      <id>753e3f13-ec42-432f-b2d2-1944dff96694</id>
      <masked>false</masked>
      <name>baseUrl</name>
   </variables>
   <variables>
      <defaultValue>'con cho'</defaultValue>
      <description></description>
      <id>44864fb8-4e2c-422e-8ee8-e14eb2431e3b</id>
      <masked>false</masked>
      <name>dog</name>
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




WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


WS.verifyElementPropertyValue(response, 'success', true)

println request.getVariables().get(&quot;productName&quot;)

println request.getVariables().get(&quot;test&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
