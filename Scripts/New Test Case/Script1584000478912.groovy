import groovy.text.GStringTemplateEngine
import internal.GlobalVariable as GlobalVariable

String a = "${GlobalVariable.tag}abc\$a"

def variables = [('GlobalVariable'): internal.GlobalVariable]

GStringTemplateEngine engine = new GStringTemplateEngine();

String b = engine.createTemplate(a).make(variables).toString()
