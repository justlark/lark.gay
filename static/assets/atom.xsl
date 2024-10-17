<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="3.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform" xmlns:atom="http://www.w3.org/2005/Atom" exclude-result-prefixes="atom">
  <xsl:output method="html" version="1.0" encoding="UTF-8" indent="yes" />
  <xsl:template match="/">
    <html xmlns="http://www.w3.org/1999/xhtml" lang="en">
      <head>
        <meta charset="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <meta name="description">
          <xsl:attribute name="content">
            <xsl:value-of select="atom:feed/atom:subtitle" />
          </xsl:attribute>
        </meta>
        <title><xsl:value-of select="atom:feed/atom:title" /> Feed</title>
        <link rel="canonical">
          <xsl:attribute name="href">
            <xsl:value-of select="atom:feed/atom:link[@rel='self']/@href" />
          </xsl:attribute>
        </link>
        <link rel="alternate" type="application/atom+xml">
          <xsl:attribute name="href">
            <xsl:value-of select="atom:feed/atom:link[@rel='self']/@href" />
          </xsl:attribute>
        </link>
        <link rel="shortcut icon">
          <xsl:attribute name="href">
            <xsl:value-of select="atom:feed/atom:icon" />
          </xsl:attribute>
        </link>
        <link rel="preconnect" href="https://fonts.googleapis.com" />
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="true" />
        <link href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:ital,wght@0,100..800;1,100..800&amp;display=swap" rel="stylesheet" />
        <link rel="stylesheet" href="/assets/index.css" />
      </head>
      <body data-theme="catppuccin-mocha">
        <main class="feed" aria-labelledby="site-title">
          <h1 id="site-title">
            <xsl:value-of select="atom:feed/atom:title" />
          </h1>
          <div class="page-lead">
            <p>
              This is a feed of log entries from Lark Space.
            </p>
            <p>
              Copy this URL into your RSS reader.
            </p>
            <p>
              <span>&lt;- </span>
              <a href="/log/">Back</a>
            </p>
          </div>
          <xsl:apply-templates select="atom:feed/atom:entry" />
        </main>
      </body>
    </html>
  </xsl:template>
  <xsl:template match="atom:feed/atom:entry">
    <article>
      <xsl:attribute name="aria-labelledby">article-title-<xsl:value-of select="position()" /></xsl:attribute>
      <h2>
        <xsl:attribute name="id">article-title-<xsl:value-of select="position()" /></xsl:attribute>
        <xsl:value-of select="atom:title" />
      </h2>
      <p>
        <span>
          <xsl:value-of select="atom:author/atom:name" />
        </span>
        <span> • </span>
        <time>
          <xsl:attribute name="datetime">
            <xsl:value-of select="atom:published" />
          </xsl:attribute>
          <xsl:call-template name="date">
            <xsl:with-param name="date" select="atom:published" />
          </xsl:call-template>
        </time>
      </p>
      <xsl:if test="atom:category">
        <ul class="tag-list">
          <xsl:for-each select="atom:category">
            <li> #<xsl:value-of select="@label" />
            </li>
          </xsl:for-each>
        </ul>
      </xsl:if>
    </article>
    <hr />
  </xsl:template>
  <xsl:template name="date">
    <xsl:param name="date" />
    <xsl:variable name="year" select="substring($date, 1, 4)" />
    <xsl:variable name="month" select="substring($date, 6, 2)" />
    <xsl:variable name="day" select="substring($date, 9, 2)" />
    <xsl:value-of select="$day" />
    <xsl:text> </xsl:text>
    <xsl:choose>
      <xsl:when test="$month=01">Jan</xsl:when>
      <xsl:when test="$month=02">Feb</xsl:when>
      <xsl:when test="$month=03">Mar</xsl:when>
      <xsl:when test="$month=04">Apr</xsl:when>
      <xsl:when test="$month=05">May</xsl:when>
      <xsl:when test="$month=06">Jun</xsl:when>
      <xsl:when test="$month=07">Jul</xsl:when>
      <xsl:when test="$month=08">Aug</xsl:when>
      <xsl:when test="$month=09">Sep</xsl:when>
      <xsl:when test="$month=10">Oct</xsl:when>
      <xsl:when test="$month=11">Nov</xsl:when>
      <xsl:when test="$month=12">Dec</xsl:when>
    </xsl:choose>
    <xsl:text> </xsl:text>
    <xsl:value-of select="$year" />
  </xsl:template>
</xsl:stylesheet>

