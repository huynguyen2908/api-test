
/**
 * This class is generated automatically by Katalon Studio and should not be modified or deleted.
 */

import com.kms.katalon.core.testobject.TestObject

import java.lang.String

import com.applitools.eyes.selenium.Eyes

import org.openqa.selenium.WebElement

import com.applitools.eyes.RectangleSize


 /**
	 * Refresh browser
	 */ 
def static "def.test.refreshBrowser"() {
    (new def.test()).refreshBrowser()
}

 /**
	 * Click element
	 * @param to Katalon test object
	 */ 
def static "def.test.clickElement"(
    	TestObject to	) {
    (new def.test()).clickElement(
        	to)
}

 /**
	 * Get all rows of HTML table
	 * @param table Katalon test object represent for HTML table
	 * @param outerTagName outer tag name of TR tag, usually is TBODY
	 * @return All rows inside HTML table
	 */ 
def static "def.test.getHtmlTableRows"(
    	TestObject table	
     , 	String outerTagName	) {
    (new def.test()).getHtmlTableRows(
        	table
         , 	outerTagName)
}

 /**
	 * Check if element present in timeout
	 * @param to Katalon test object
	 * @param timeout time to wait for element to show up
	 * @return true if element present, otherwise false
	 */ 
def static "def.test.isElementPresent_Mobile"(
    	TestObject to	
     , 	int timeout	) {
    (new def.test()).isElementPresent_Mobile(
        	to
         , 	timeout)
}

 /**
	 * Get mobile driver for current session
	 * @return mobile driver for current session
	 */ 
def static "def.test.getCurrentSessionMobileDriver"() {
    (new def.test()).getCurrentSessionMobileDriver()
}

 /**
	 * Send request and verify status code
	 * @param request request object, must be an instance of RequestObject
	 * @param expectedStatusCode
	 * @return a boolean to indicate whether the response status code equals the expected one
	 */ 
def static "def.test.verifyStatusCode"(
    	TestObject request	
     , 	int expectedStatusCode	) {
    (new def.test()).verifyStatusCode(
        	request
         , 	expectedStatusCode)
}

 /**
	 * Add Header basic authorization field,
	 * this field value is Base64 encoded token from user name and password
	 * @param request object, must be an instance of RequestObject
	 * @param username username
	 * @param password password
	 * @return the original request object with basic authorization header field added
	 */ 
def static "def.test.addBasicAuthorizationProperty"(
    	TestObject request	
     , 	String username	
     , 	String password	) {
    (new def.test()).addBasicAuthorizationProperty(
        	request
         , 	username
         , 	password)
}

 /**
	 * Refresh browser
	 */ 
def static "abc.abcde.refreshBrowser"() {
    (new abc.abcde()).refreshBrowser()
}

 /**
	 * Click element
	 * @param to Katalon test object
	 */ 
def static "abc.abcde.clickElement"(
    	TestObject to	) {
    (new abc.abcde()).clickElement(
        	to)
}

 /**
	 * Get all rows of HTML table
	 * @param table Katalon test object represent for HTML table
	 * @param outerTagName outer tag name of TR tag, usually is TBODY
	 * @return All rows inside HTML table
	 */ 
def static "abc.abcde.getHtmlTableRows"(
    	TestObject table	
     , 	String outerTagName	) {
    (new abc.abcde()).getHtmlTableRows(
        	table
         , 	outerTagName)
}

 /**
	 * Check if element present in timeout
	 * @param to Katalon test object
	 * @param timeout time to wait for element to show up
	 * @return true if element present, otherwise false
	 */ 
def static "abc.abcde.isElementPresent_Mobile"(
    	TestObject to	
     , 	int timeout	) {
    (new abc.abcde()).isElementPresent_Mobile(
        	to
         , 	timeout)
}

 /**
	 * Get mobile driver for current session
	 * @return mobile driver for current session
	 */ 
def static "abc.abcde.getCurrentSessionMobileDriver"() {
    (new abc.abcde()).getCurrentSessionMobileDriver()
}

 /**
	 * Send request and verify status code
	 * @param request request object, must be an instance of RequestObject
	 * @param expectedStatusCode
	 * @return a boolean to indicate whether the response status code equals the expected one
	 */ 
def static "abc.abcde.verifyStatusCode"(
    	TestObject request	
     , 	int expectedStatusCode	) {
    (new abc.abcde()).verifyStatusCode(
        	request
         , 	expectedStatusCode)
}

 /**
	 * Add Header basic authorization field,
	 * this field value is Base64 encoded token from user name and password
	 * @param request object, must be an instance of RequestObject
	 * @param username username
	 * @param password password
	 * @return the original request object with basic authorization header field added
	 */ 
def static "abc.abcde.addBasicAuthorizationProperty"(
    	TestObject request	
     , 	String username	
     , 	String password	) {
    (new abc.abcde()).addBasicAuthorizationProperty(
        	request
         , 	username
         , 	password)
}


def static "com.kms.katalon.keyword.applitools.BasicKeywords.checkElement"(
    	Eyes eyes	
     , 	WebElement element	) {
    (new com.kms.katalon.keyword.applitools.BasicKeywords()).checkElement(
        	eyes
         , 	element)
}


def static "com.kms.katalon.keyword.applitools.BasicKeywords.checkWindow"(
    	String testName	) {
    (new com.kms.katalon.keyword.applitools.BasicKeywords()).checkWindow(
        	testName)
}


def static "com.kms.katalon.keyword.applitools.BasicKeywords.checkTestObject"(
    	TestObject testObject	
     , 	String testName	) {
    (new com.kms.katalon.keyword.applitools.BasicKeywords()).checkTestObject(
        	testObject
         , 	testName)
}


def static "com.kms.katalon.keyword.applitools.EyesKeywords.eyesOpen"(
    	String testName	
     , 	RectangleSize viewportSize	) {
    (new com.kms.katalon.keyword.applitools.EyesKeywords()).eyesOpen(
        	testName
         , 	viewportSize)
}


def static "com.kms.katalon.keyword.applitools.EyesKeywords.eyesInit"() {
    (new com.kms.katalon.keyword.applitools.EyesKeywords()).eyesInit()
}


def static "com.kms.katalon.keyword.applitools.EyesKeywords.eyesOpenWithBaseline"(
    	String baselineName	
     , 	String testName	
     , 	RectangleSize viewportSize	) {
    (new com.kms.katalon.keyword.applitools.EyesKeywords()).eyesOpenWithBaseline(
        	baselineName
         , 	testName
         , 	viewportSize)
}


def static "com.kms.katalon.keyword.applitools.EyesKeywords.eyesClose"(
    	Eyes eyes	) {
    (new com.kms.katalon.keyword.applitools.EyesKeywords()).eyesClose(
        	eyes)
}
