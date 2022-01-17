<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>custom-with-body</name>
   <tag></tag>
   <elementGuidId>32df57ab-9714-4cde-b444-9ff8c26b54f2</elementGuidId>
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
   <restRequestMethod>CUSTOM</restRequestMethod>
   <restUrl>https://enpdym14txx78.x.pipedream.net?qp1=val1&amp;qp2=${GlobalVariable.test}&amp;qp3=${myVariable1}&amp;qp4=${myVariable2}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'myVariable1_OR'</defaultValue>
      <description></description>
      <id>a748393d-67df-4e19-bf9d-8777a1dd688b</id>
      <masked>false</masked>
      <name>myVariable1</name>
   </variables>
   <variables>
      <defaultValue>'myVariable2_OR'</defaultValue>
      <description></description>
      <id>4eb34a45-96de-4490-bb7f-16cd360d3e1f</id>
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
