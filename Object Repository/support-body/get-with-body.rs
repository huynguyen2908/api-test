<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>get-with-body</name>
   <tag></tag>
   <elementGuidId>cc9b17bd-2fe0-4893-8ccf-379cc2d3b101</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;field1\&quot;: \&quot;the value is ${GlobalVariable.productName}\&quot;,\n    \&quot;field2\&quot;: \&quot;value2\&quot;\n}&quot;,
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
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://enkwhxuow580j.x.pipedream.net?qp1=val1&amp;qp2=${GlobalVariable.test}&amp;qp3=${myVariable1}&amp;qp4=${myVariable2}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'myVariable1_OR'</defaultValue>
      <description></description>
      <id>794cdec3-6b2b-444c-a63d-ee778caee48a</id>
      <masked>false</masked>
      <name>myVariable1</name>
   </variables>
   <variables>
      <defaultValue>'myVariable2_OR'</defaultValue>
      <description></description>
      <id>bf4dafc1-b56a-466d-80a8-450b1aacc1c9</id>
      <masked>false</masked>
      <name>myVariable2</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import org.apache.commons.io.IOUtils

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.verification.WSResponseManager

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

println IOUtils.toString(request.getBodyContent().getInputStream())

println response.getResponseText()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
