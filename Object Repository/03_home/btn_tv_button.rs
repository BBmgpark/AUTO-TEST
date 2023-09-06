<?xml version="1.0" encoding="UTF-8"?>
<MobileElementEntity>
   <description>[영유아검진 작성]</description>
   <name>btn_tv_button</name>
   <tag></tag>
   <elementGuidId>2276c0b1-d1d0-4b77-b8e4-5a62eff91a6e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>resource-id</name>
      <type>Main</type>
      <value>com.bbros.sayup.debug:id/tv_button</value>
      <webElementGuid>94fcded6-30be-4ba8-b794-88180a640fae</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>${text}</value>
      <webElementGuid>a76a435c-1caa-40ab-a6c5-c6a659afd65c</webElementGuid>
   </webElementProperties>
   <locator>//*[@resource-id = 'com.bbros.sayup.debug:id/tv_button' and (@text = '${text}' or . = '${text}')]</locator>
   <locatorStrategy>XPATH</locatorStrategy>
</MobileElementEntity>
