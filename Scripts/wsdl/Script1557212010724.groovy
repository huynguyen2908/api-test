import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS

import static org.assertj.core.api.Assertions.*
import groovy.util.XmlSlurper

ResponseObject response = WS.sendRequest(findTestObject('wsdl', [('operandA'): 80]))

WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

println WS.getElementPropertyValue(response, 'AddResponse.AddResult')

WS.verifyElementPropertyValue(response, 'AddResponse.AddResult', 100)

String bodyContent = response.getResponseBodyContent()

def AddResponse = new XmlSlurper().parseText(bodyContent)

response = WS.sendRequestAndVerify(findTestObject('wsdl', [('operandA'): 10]))

WS.verifyResponseStatusCode(response, 200)

WS.verifyElementPropertyValue(response, 'AddResponse.AddResult', 30)

def AddResult = AddResponse.AddResult
//
println AddResponse.AddResult.text()
//
println WS.getElementText(response, 'AddResponse.AddResult')