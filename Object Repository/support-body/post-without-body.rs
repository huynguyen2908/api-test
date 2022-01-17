<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>post-without-body</name>
   <tag></tag>
   <elementGuidId>e70cc35b-1112-49e4-9add-ac869a457fdb</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
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
      <id>1c527373-a765-4a81-b686-888ca7382a18</id>
      <masked>false</masked>
      <name>myVariable1</name>
   </variables>
   <variables>
      <defaultValue>'myVariable2_OR'</defaultValue>
      <description></description>
      <id>abc9ac61-3eaa-45ea-aa0b-381b22105cdf</id>
      <masked>false</masked>
      <name>myVariable2</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.verification.WSResponseManager

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

println request.getRestUrl()

println response.getResponseText()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
