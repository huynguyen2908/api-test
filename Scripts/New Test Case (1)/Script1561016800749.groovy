import com.kms.katalon.core.testobject.ConditionType
import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.TestObjectProperty
import com.kms.katalon.core.testobject.impl.HttpTextBodyContent
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS

import internal.GlobalVariable as GlobalVariable

'1a. prepare request Body - set data for plugin'
HashMap<String, Object> plugin = new HashMap<>()
//set plugin name
plugin.put('name', "abc")

//set plugin license type - eithor "Free" or "Paid"
plugin.put('licenseType', "Paid")

//set plugin license type - eithor "Katalon Plugin" or "Custom Keyword"
plugin.put('pluginType', "Katalon Plugin")

def licenseType = "Paid"
//set plugin price, optional (required when license type = Paid)
if ("${licenseType}" == "Paid") {
	HashMap prices = new HashMap()
	prices.put('individual', "10000")
	prices.put('team', "10000")
	prices.put('corporate', "10000")
	plugin.put('price', "10000")
}

//Convert map into Json
def jsonBodyContent = new groovy.json.JsonBuilder()
jsonBodyContent plugin

//Convert Json into String
String bodyContent =  jsonBodyContent.toString()

//Create body content property
HttpTextBodyContent httpBodyContent = new HttpTextBodyContent(bodyContent);

'1b. prepare request Header - create header properties'
List<TestObjectProperty> httpHeaderProperties = new ArrayList<>()
httpHeaderProperties.add(new TestObjectProperty("Content-Type", ConditionType.EQUALS, 'application/json'))
httpHeaderProperties.add(new TestObjectProperty("username", ConditionType.EQUALS, "abc"))
httpHeaderProperties.add(new TestObjectProperty("password", ConditionType.EQUALS, "def"))

'1. prepare WS request'
RequestObject scriptedWSO = new RequestObject('Create a new plugin')
scriptedWSO.setBodyContent(httpBodyContent)
scriptedWSO.setServiceType('REST')
scriptedWSO.setHttpHeaderProperties(httpHeaderProperties)
scriptedWSO.setRestUrl("http://www.google.com")
scriptedWSO.setRestRequestMethod('POST')

'2. send the request and get the response'
response = WS.sendRequest(scriptedWSO)

'3. get returned plugin id'
//def jsonSlurper = new JsonSlurper()
//def newPlugin = jsonSlurper.parseText(response.getResponseText())
//return newPlugin.id