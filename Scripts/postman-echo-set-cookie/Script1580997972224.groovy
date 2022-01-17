import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS

RequestObject request = findTestObject('Object Repository/postman-echo-set-cookie')

ResponseObject response = WS.sendRequest(request)

def responseCookies = response.getHeaderFields()

def cookieHeaders = responseCookies['set-cookie']

println "abc"