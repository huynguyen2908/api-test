package internal

import com.kms.katalon.core.configuration.RunConfiguration
import com.kms.katalon.core.main.TestCaseMain


/**
 * This class is generated automatically by Katalon Studio and should not be modified or deleted.
 */
public class GlobalVariable {
     
    /**
     * <p></p>
     */
    public static Object xpath
     
    /**
     * <p></p>
     */
    public static Object tag
     
    /**
     * <p></p>
     */
    public static Object productName
     
    /**
     * <p></p>
     */
    public static Object wsdlUrl
     
    /**
     * <p></p>
     */
    public static Object operandB
     
    /**
     * <p></p>
     */
    public static Object url
     
    /**
     * <p></p>
     */
    public static Object test
     
    /**
     * <p></p>
     */
    public static Object dummy
     
    /**
     * <p></p>
     */
    public static Object contentType
     
    /**
     * <p></p>
     */
    public static Object aaa
     

    static {
        try {
            def selectedVariables = TestCaseMain.getGlobalVariables("default")
			selectedVariables += TestCaseMain.getGlobalVariables(RunConfiguration.getExecutionProfile())
            selectedVariables += TestCaseMain.getParsedValues(RunConfiguration.getOverridingParameters())
    
            xpath = selectedVariables['xpath']
            tag = selectedVariables['tag']
            productName = selectedVariables['productName']
            wsdlUrl = selectedVariables['wsdlUrl']
            operandB = selectedVariables['operandB']
            url = selectedVariables['url']
            test = selectedVariables['test']
            dummy = selectedVariables['dummy']
            contentType = selectedVariables['contentType']
            aaa = selectedVariables['aaa']
            
        } catch (Exception e) {
            TestCaseMain.logGlobalVariableError(e)
        }
    }
}
