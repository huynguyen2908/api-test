<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>002_create_VM</name>
   <tag></tag>
   <elementGuidId>0156bbe5-fcdf-493c-8caf-e5e8d3ef31ae</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n   \&quot;links\&quot;:[\n      {\n         \&quot;title\&quot;:\&quot;WINSRV2016-STD-SingleSiteB\&quot;,\n         \&quot;rel\&quot;:\&quot;virtualmachinetemplate\&quot;,\n         \&quot;type\&quot;:\&quot;application/vnd.abiquo.virtualmachinetemplate+json\&quot;,\n         \&quot;href\&quot;:\&quot;https://ss009620.itoper.local:443/api/admin/enterprises/3/datacenterrepositories/1/virtualmachinetemplates/84\&quot;\n      },\n      {\n         \&quot;title\&quot;:\&quot;Small\&quot;,\n         \&quot;rel\&quot;:\&quot;hardwareprofile\&quot;,\n         \&quot;type\&quot;:\&quot;application/vnd.abiquo.hardwareprofile+json\&quot;,\n         \&quot;href\&quot;:\&quot;https://ss009620.itoper.local:443/api/cloud/locations/1/hardwareprofiles/8\&quot;\n      }\n   ],\n   \&quot;label\&quot;:\&quot;TestVM\&quot;,\n   \&quot;vdrpEnabled\&quot;:true\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/vnd.abiquo.virtualmachine+json;version=4.7</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/vnd.abiquo.virtualmachine+json;version=4.7</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization:type</name>
      <type>Main</type>
      <value>OAuth 1.0</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization:oauth_consumer_key</name>
      <type>Main</type>
      <value>71341327-ee24-452d-8d86-4808a8e0dxxx</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization:oauth_consumer_secret</name>
      <type>Main</type>
      <value>fmWozwWBT3pFL3HhXuFaBna3uItjJvkTAhoxXXX</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization:oauth_signature_method</name>
      <type>Main</type>
      <value>HMAC-SHA1</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization:oauth_token</name>
      <type>Main</type>
      <value>0915d553-98d0-4d1b-afb2-f5ef7xxxxx</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization:oauth_token_secret</name>
      <type>Main</type>
      <value>R2VucCE4Wbb7n4582IDF0Ju/OnYo4B43KLFk3ofB3iak5s56K/WGNuMb1z2Lt8THvvW04amr0V5UQUIxxxxxxxxxxxxxxx</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://en9rnyzdnbea.x.pipedream.net/</restUrl>
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
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
