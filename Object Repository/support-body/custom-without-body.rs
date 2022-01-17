<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>custom-without-body</name>
   <tag></tag>
   <elementGuidId>92141441-b467-454d-8e82-0784a9916a55</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
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
      <id>036b5890-e722-4b71-81a8-4ad3e7d15ebc</id>
      <masked>false</masked>
      <name>myVariable1</name>
   </variables>
   <variables>
      <defaultValue>'myVariable2_OR'</defaultValue>
      <description></description>
      <id>c5d11ec2-876e-4694-93fb-a16755a40fbe</id>
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

println request.getRestUrl()

println response.getResponseText()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
