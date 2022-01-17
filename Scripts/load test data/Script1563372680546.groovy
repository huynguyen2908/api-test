import static com.kms.katalon.core.testdata.TestDataFactory.findTestData

import com.kms.katalon.core.util.KeywordUtil

def testData = findTestData("user-data")

KeywordUtil.logInfo("getColumnNumbers: " + testData.getColumnNumbers())

KeywordUtil.logInfo("getRowNumbers: " + testData. getRowNumbers())