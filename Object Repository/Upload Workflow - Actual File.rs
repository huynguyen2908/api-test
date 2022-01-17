<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Upload Workflow - Actual File</name>
   <tag></tag>
   <elementGuidId>5ef674cd-5656-48f9-b90b-1b2144236c71</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;filePath&quot;: &quot;Object Repository/Anamolies Workflow.zip&quot;,
  &quot;fileSize&quot;: 986,
  &quot;contentType&quot;: &quot;application/zip&quot;
}</httpBodyContent>
   <httpBodyType>file</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/zip</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>bearer ${bearer_token}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://enq2h0odht2bj.x.pipedream.net/</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>f45e1988-2d92-4291-88c0-ec5ad2bcbeac</id>
      <masked>false</masked>
      <name>bearer_token</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ee747e1e-667f-4c63-b7df-9cc270b2f247</id>
      <masked>false</masked>
      <name>job_id</name>
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





GlobalVariable.variable</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
