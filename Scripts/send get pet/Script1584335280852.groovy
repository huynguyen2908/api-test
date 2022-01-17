import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS

RequestObject request = findTestObject('Object Repository/getPet')

ResponseObject response = WS.sendRequest(request)

println response.getResponseText()

println response.getResponseBodyContent()