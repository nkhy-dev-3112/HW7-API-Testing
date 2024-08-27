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
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import groovy.json.JsonSlurper

TestObject tobj = null

switch (type) {
	case "DV":
		tobj = findTestObject('Create Channel/DV', ['channelName': channelName, 'description': description, 'userIds': userIds, 'creatorId': creatorId, 'type': typeChannel, 'groupId': groupId])
		break;
	case "AU-1":
		tobj = findTestObject('Create Channel/AU1', ['channelName': channelName, 'description': description, 'userIds': userIds, 'creatorId': creatorId, 'type': typeChannel, 'groupId': groupId])
		break;
	case "AU-2":
		tobj = findTestObject('Create Channel/AU2', ['channelName': channelName, 'description': description, 'userIds': userIds, 'creatorId': creatorId, 'type': typeChannel, 'groupId': groupId])
		break;
	case "null1":
		tobj = findTestObject('Create Channel/NULL1', ['description': description])
		break;
	case "null2":
		tobj = findTestObject('Create Channel/NULL2', ['channelName': channelName, 'userIds': userIds, 'creatorId': creatorId, 'type': typeChannel, 'groupId': groupId])
		break;
	case "DV-1":
		tobj = findTestObject('Create Channel/DV1', ['channelName': channelName, 'description': description, 'userIds': userIds, 'creatorId': creatorId, 'type': typeChannel, 'groupId': groupId])
		break;
	default:
		break;
}


if (tobj !== null) {
	def res = WS.sendRequest(tobj)

	WS.verifyResponseStatusCode(res, Integer.parseInt(expectedCode))

}