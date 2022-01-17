import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import org.apache.commons.io.FileUtils

import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS

ResponseObject responseObject = WS.sendRequest(findTestObject('my-image'))
ByteArrayOutputStream outputStream = new ByteArrayOutputStream()
responseObject.getBodyContent().writeTo(outputStream)
File file = new File("/Users/huynguyen/Desktop/my-image.png")
if (file.exists()) {
  file.delete()
}
file.createNewFile()
FileUtils.writeByteArrayToFile(file, outputStream.toByteArray())
