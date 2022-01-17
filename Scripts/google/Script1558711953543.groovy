import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS

ResponseObject response = WS.sendRequest(findTestObject('test object của tui quá xuất sắc', ['url' : 'https://www.google.com']))
println response.getStatusCode()
