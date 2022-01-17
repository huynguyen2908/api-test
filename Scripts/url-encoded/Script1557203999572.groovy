import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.google.gson.GsonBuilder
import com.google.gson.Gson
import com.google.gson.JsonElement
import com.google.gson.JsonSerializationContext
import com.google.gson.JsonSerializer
import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.testobject.HttpBodyContent
import java.lang.reflect.Type

import java.lang.management.ManagementFactory

import java.lang.management.RuntimeMXBean

RequestObject requestObject = findTestObject('url-encoded')
ResponseObject response = WS.sendRequestAndVerify(requestObject)
WS.verifyResponseStatusCode(response, 200)

RuntimeMXBean runtimeMxBean = ManagementFactory.getRuntimeMXBean()
List<String> arguments = runtimeMxBean.getInputArguments()

println arguments

println System.getProperty("test")

println System.getProperty("test1")