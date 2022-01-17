import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import org.apache.commons.io.IOUtils

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.testobject.TestObjectProperty
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS

RequestObject request = findTestObject('Object Repository/support-body/custom-with-body', [('myVariable2'): "myVariable2_TC"])

println "REQUEST URL: " + request.getRestUrl()

println "HTTP METHOD: " + request.getRestRequestMethod()

String variables = ""
for (String variableName : request.getVariables().keySet()) {
    variables += variableName + ":" + request.getVariables().get(variableName) + "\n"
}
println "VARIABLES:\n" + variables

String queryParams = ""
for (TestObjectProperty property : request.getRestParameters()) {
    queryParams += property.getName() + ":" + property.getValue() + "\n"
}
println "QUERY PARAMETERS:\n" + queryParams

String headers = ""
for (TestObjectProperty property : request.getHttpHeaderProperties()) {
    headers += property.getName() + ":" + property.getValue() + "\n"
}
println "HEADERS:\n" + headers

println "BODY CONTENT:\n" + IOUtils.toString(request.getBodyContent().getInputStream())

println "VERIFICATION SCRIPT: " + request.getVerificationScript()

ResponseObject response = WS.sendRequest(request)

println "STATUS CODE: " + response.getStatusCode()

println "RESPONSE BODY CONTENT:\n" + response.getResponseText()
