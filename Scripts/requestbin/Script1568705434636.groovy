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
import internal.GlobalVariable as GlobalVariable
import com.kms.katalon.core.testobject.ResponseObject as ResponseObject
import com.kms.katalon.core.testobject.RequestObject as RequestObject
import static org.assertj.core.api.Assertions.*

RequestObject requestObject = findTestObject('requestbin', [('productName') : productName, ('dog'): dog])

ResponseObject response1 = WS.sendRequest(requestObject)

WS.verifyResponseStatusCode(response1, 200)

assertThat(response1.getStatusCode()).isEqualTo(200)

WS.verifyElementPropertyValue(response1, 'success', true)

ResponseObject response2 = WS.sendRequestAndVerify(requestObject)

WS.verifyResponseStatusCode(response2, 200)

assertThat(response2.getStatusCode()).isEqualTo(200)

WS.verifyElementPropertyValue(response2, 'success', true)

