<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>wsdl</name>
   <tag></tag>
   <elementGuidId>4819e2b4-c112-400c-9cc6-bcdfcf2c31fe</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>test</name>
      <type>Main</type>
      <value>abc</value>
   </httpHeaderProperties>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;?xml version=&quot;1.0&quot; encoding=&quot;UTF-8&quot;?>&lt;SOAP-ENV:Envelope xmlns:SOAP-ENV=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:tns=&quot;http://tempuri.org/&quot;>
  &lt;SOAP-ENV:Header/>
  &lt;SOAP-ENV:Body>
    &lt;tns:Add>
      &lt;tns:intA>${operandA}&lt;/tns:intA>
      &lt;tns:intB>${operandB}&lt;/tns:intB>
    &lt;/tns:Add>
  &lt;/SOAP-ENV:Body>
&lt;/SOAP-ENV:Envelope>
</soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP12</soapRequestMethod>
   <soapServiceFunction>Add</soapServiceFunction>
   <variables>
      <defaultValue>10</defaultValue>
      <description></description>
      <id>2d36b9e0-c68c-47da-a73f-d55643046ce7</id>
      <masked>false</masked>
      <name>operandA</name>
   </variables>
   <variables>
      <defaultValue>20</defaultValue>
      <description></description>
      <id>1f07adc5-6c57-4d0c-9f06-1b247fa54700</id>
      <masked>false</masked>
      <name>operandB</name>
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

WS.verifyElementPropertyValue(response, 'AddResponse.AddResult', 30)

println request.getVariables().get(&quot;operandA&quot;)

println request.getVariables().get(&quot;operandB&quot;)

println WS.getElementText(response, &quot;AddResponse&quot;)
//
//String bodyContent = response.getResponseBodyContent()
//
//def root = new JsonSlurper().parseText(bodyContent)
//
//println root.AddResponse.AddResult.getClass()</verificationScript>
   <wsdlAddress>http://localhost/calculator.asmx?WSDL</wsdlAddress>
</WebServiceRequestEntity>
