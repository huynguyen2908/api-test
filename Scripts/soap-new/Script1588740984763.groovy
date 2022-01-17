import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS

//RequestObject requestObject = findTestObject('test-SOAP/soap-legacy', [('variable') : '123', ('variable_0') : '456', ('variable_1') : 91011])

//RequestObject requestObject = findTestObject('Object Repository/Divide')
RequestObject requestObject = findTestObject('Object Repository/Subtract')
 
assert requestObject.useServiceInfoFromWsdl()

requestObject.setUseServiceInfoFromWsdl(false)
//
assert !requestObject.useServiceInfoFromWsdl()

requestObject.setSoapServiceEndpoint("http://www.dneonline.com/calculator.asmx")

ResponseObject response = WS.sendRequest(requestObject)

println WS.verifyResponseStatusCode(response, 200)


