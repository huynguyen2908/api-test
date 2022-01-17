import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable

import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import org.apache.commons.io.IOUtils

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.testobject.TestObjectProperty
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS

RequestObject request = findTestObject('Object Repository/support-body/get-without-body', [('myVariable2'): "myVariable2_TC"])

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

//println "BODY CONTENT:\n" + IOUtils.toString(request.getBodyContent().getInputStream())

println "VERIFICATION SCRIPT: " + request.getVerificationScript()

ResponseObject response = WS.sendRequestAndVerify(request)

println "STATUS CODE: " + response.getStatusCode()

println "RESPONSE BODY CONTENT:\n" + response.getResponseText()
