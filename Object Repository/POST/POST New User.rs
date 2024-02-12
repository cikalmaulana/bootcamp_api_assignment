<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST New User</name>
   <tag></tag>
   <elementGuidId>a4abbe11-c63e-458e-8469-f3d47ff58c3a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;name\&quot;: \&quot;Bukan Dika\&quot;,\n    \&quot;username\&quot;: \&quot;Bukan_Dika_69\&quot;,\n    \&quot;email\&quot;: \&quot;IniBukanDika@gmail.com\&quot;,\n    \&quot;address\&quot;: {\n      \&quot;street\&quot;: \&quot;Kebun Naga\&quot;,\n      \&quot;suite\&quot;: \&quot;Naga 879\&quot;,\n      \&quot;city\&quot;: \&quot;Banyuwangi\&quot;,\n      \&quot;zipcode\&quot;: \&quot;90566-7771\&quot;,\n      \&quot;geo\&quot;: {\n        \&quot;lat\&quot;: \&quot;-43.9509\&quot;,\n        \&quot;lng\&quot;: \&quot;-34.4618\&quot;\n      }\n    },\n    \&quot;phone\&quot;: \&quot;010-692-6593 x09125\&quot;,\n    \&quot;website\&quot;: \&quot;buahnagadika.com\&quot;,\n    \&quot;company\&quot;: {\n      \&quot;name\&quot;: \&quot;PT Buah Naga Dika\&quot;,\n      \&quot;catchPhrase\&quot;: \&quot;Proactive didactic contingency\&quot;,\n      \&quot;bs\&quot;: \&quot;synergize scalable supply-chains\&quot;\n  \t}\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>0fc09a8d-149f-42de-866a-7e7bc63ed99f</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.0.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://jsonplaceholder.typicode.com/users</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 201)

assertThat(response.getStatusCode()).isEqualTo(201)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
