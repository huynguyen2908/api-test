import com.kms.katalon.core.testobject.ConditionType
import com.kms.katalon.core.testobject.FormDataBodyParameter
import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.testobject.RestRequestObjectBuilder
import com.kms.katalon.core.testobject.TestObjectProperty
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS

import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

//String username = "Henry"
RequestObject request = findTestObject('form-data', [('username'): username])
println request.getVariables().get("username")
ResponseObject response = WS.sendRequestAndVerify(request)
println response.getStatusCode()

//RestRequestObjectBuilder builder = new RestRequestObjectBuilder();
//
//List<TestObjectProperty> headers = new ArrayList<>();
//
//headers.add(new TestObjectProperty("Content-Type", ConditionType.EQUALS, "multipart/form-data"));
//
//List<FormDataBodyParameter> params = new ArrayList<>();
//
//FormDataBodyParameter paramUsername = new FormDataBodyParameter("username", "David", "Text");
//params.add(paramUsername);
//
//FormDataBodyParameter paramPassword = new FormDataBodyParameter("password", "123", "Text");
//params.add(paramPassword);
//
//FormDataBodyParameter paramGender = new FormDataBodyParameter("gender", "MALE", "Text");
//params.add(paramGender);
//
//FormDataBodyParameter paramAge = new FormDataBodyParameter("age", "76", "Text");
//params.add(paramAge);
//
//File file = new File("/Users/huynguyen/Downloads/Katalon OEM license - 05_09_2019.docx");
//println file.getAbsolutePath()
//println file.exists()
//
//FormDataBodyParameter paramAvatar = new FormDataBodyParameter("avatar", "/Users/huynguyen/Downloads/Katalon OEM license - 05_09_2019.docx", "File");
//params.add(paramAvatar);
//
//RequestObject requestObject = builder.withRestRequestMethod("POST")
//	.withRestUrl("https://en60vd10xwj6r.x.pipedream.net/")
//	.withHttpHeaders(headers)
//	.withMultipartFormDataBodyContent(params)
//	.build();
//
//ResponseObject responseObject = WS.sendRequest(requestObject);
//
//
//println responseObject.getResponseBodyContent()