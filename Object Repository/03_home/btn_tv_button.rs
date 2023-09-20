<?xml version="1.0" encoding="UTF-8"?>
<MobileElementEntity>
   <description>[영유아검진 작성]</description>
   <name>btn_tv_button</name>
   <tag></tag>
   <elementGuidId>01cfd05f-4871-4443-bb23-bc7e13d88d01</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>resource-id</name>
      <type>Main</type>
      <value>com.bbros.sayup.debug:id/btn_large</value>
      <webElementGuid>01f3f67a-4bc6-4b37-a77e-d16cf6247ceb</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>${text}</value>
      <webElementGuid>8624767e-b4a3-44df-a635-d822d7d71a56</webElementGuid>
   </webElementProperties>
   <locator>//*[@resource-id = 'com.bbros.sayup.debug:id/btn_large' and (@text = '${text}' or . = '${text}')]</locator>
   <locatorStrategy>XPATH</locatorStrategy>
</MobileElementEntity>
