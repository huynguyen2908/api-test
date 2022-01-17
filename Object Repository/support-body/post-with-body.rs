<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>post-with-body</name>
   <tag></tag>
   <elementGuidId>93919906-3e7c-407e-9beb-588b275e10fc</elementGuidId>
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
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://enpdym14txx78.x.pipedream.net?qp1=val1&amp;qp2=${GlobalVariable.test}&amp;qp3=${myVariable1}&amp;qp4=${myVariable2}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'myVariable1_OR'</defaultValue>
      <description></description>
      <id>a6bfd6fb-78f9-4401-85c4-1141093ed8df</id>
      <masked>false</masked>
      <name>myVariable1</name>
   </variables>
   <variables>
      <defaultValue>'myVariable2_OR'</defaultValue>
      <description></description>
      <id>7e7e5283-2e3b-46bd-a5bd-de6337fb3400</id>
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
