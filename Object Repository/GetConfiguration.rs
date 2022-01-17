<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GetConfiguration</name>
   <tag></tag>
   <elementGuidId>7c6142a4-1098-4c5b-a426-c3e27a43c0f0</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Cookie</name>
      <type>Main</type>
      <value>JSESSIONID=YTQ5YTkwOGQtZjczZC00YWQ0LWJmOGMtYjcwY2VhMWU2OGJk</value>
   </httpHeaderProperties>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;?xml version=&quot;1.0&quot; encoding=&quot;UTF-8&quot;?>&lt;SOAP-ENV:Envelope xmlns:SOAP-ENV=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:tns=&quot;http://www.eterra.com/public/services/data/DataProvider/&quot;>
  &lt;SOAP-ENV:Header/>
  &lt;SOAP-ENV:Body>
    &lt;tns:GetConfiguration>
      &lt;tns:parameters>?&lt;/tns:parameters>
    &lt;/tns:GetConfiguration>
  &lt;/SOAP-ENV:Body>
&lt;/SOAP-ENV:Envelope>
</soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP12</soapRequestMethod>
   <soapServiceFunction>GetConfiguration</soapServiceFunction>
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
   <wsdlAddress>https://en4toq3jhde1o.x.pipedream.net/eterra-ws/soap/DataProvider?wsdl</wsdlAddress>
</WebServiceRequestEntity>
