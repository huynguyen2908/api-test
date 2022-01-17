import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.configuration.RunConfiguration
import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject as ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS

RequestObject request = findTestObject('follow-redirects')
request.setFollowRedirects(false)
ResponseObject response = WS.sendRequest(request)

println(response.getStatusCode())

