<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>mocky</name>
   <tag></tag>
   <elementGuidId>ec0c7621-e131-414c-83ed-c24abba4c9e1</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://www.mocky.io/v2/5df20f213100000e009a2f77</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

println WS.getElementPropertyValue(response, &quot;a.b[0].c&quot;)
//	
//println WS.getElementsCount(response, &quot;a.b&quot;)

println WS.getElementText(response, &quot;a&quot;)

//println WS.getResponseStatusCode(response)
//
//String bodyContent = response.getResponseBodyContent()
////
//def root = new JsonSlurper().parseText(bodyContent)
//
//println root.a.text()
////
//println bodyContent</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
