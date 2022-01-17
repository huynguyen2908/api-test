import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS

import groovy.json.JsonSlurper

ResponseObject response = WS.sendRequest(findTestObject('mocky'))

String bodyContent = response.getResponseBodyContent()
//
def root = new JsonSlurper().parseText(bodyContent)

def x = root.b

println root.b.text()
