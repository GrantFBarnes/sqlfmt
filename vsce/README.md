# sqlfmt - SQL Format

This extension allows you to utilize [sqlfmt](https://github.com/GrantFBarnes/sqlfmt) from within VS Code.

## Operation

This extension is setup as a language formatter extension.
Meaning it can be [set](https://code.visualstudio.com/docs/configure/settings#_language-specific-editor-settings) as the default formatter for `sql` files.
This gives you all the standard formatting in VS Code, such as formatting entire files or just highlighted sections.

For situations where you want to format SQL inside another file type, such as a string in another programming language, you can use the `sqlfmt - Format SQL` [command](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette).
This will run the program against the entire file or any highlighted text within any file type and replace it with the formatted result.

## Settings

The list of settings can be seen in the `FEATURES` -> `Settings` tab of the [extension details page](https://code.visualstudio.com/docs/editor/extension-marketplace#_extension-details).

## Examples

Below are some examples of how various settings change the formatted result.
All examples use the following as the input SQL:

```sql
With Cte1
As      (Select   Column1,Column2 ,Column3, Column4   From   Table1   )
,
Cte2 as (     select Column1
,Column2
FROM Table2)
  Select
  Distinct
Cte1.*,Cte2.Column2
From Cte1 inner join Cte2
ON   Cte2.Column1 =Cte1.Column1
Order By
 Cte1.Column2
```

<table>
<thead>
<tr>
<th>Settings</th>
<th>Result</th>
</tr>
</thead>
<tbody>
<tr>
<td>
<ul>
<li>(default)</li>
</ul>
</td>
<td>

```sql
With Cte1
    As (Select Column1, Column2, Column3, Column4 From Table1)
    ,
    Cte2 as (select Column1
            , Column2
        FROM Table2)
Select
    Distinct
    Cte1.*, Cte2.Column2
From Cte1 inner join Cte2
    ON Cte2.Column1 = Cte1.Column1
Order By
    Cte1.Column2
```

</td>
</tr>
<tr>
<td>
<ul>
<li>Replace Newlines</li>
</ul>
</td>
<td>

```sql
With Cte1 As (Select Column1, Column2, Column3, Column4 From Table1),
    Cte2 as (select Column1, Column2 FROM Table2)
Select Distinct
    Cte1.*,
    Cte2.Column2
From Cte1
    inner join Cte2 ON Cte2.Column1 = Cte1.Column1
Order By Cte1.Column2
```

</td>
</tr>
<tr>
<td>
<ul>
<li>Replace Newlines</li>
<li>Uppercase</li>
</ul>
</td>
<td>

```sql
WITH Cte1 AS (SELECT Column1, Column2, Column3, Column4 FROM Table1),
    Cte2 AS (SELECT Column1, Column2 FROM Table2)
SELECT DISTINCT
    Cte1.*,
    Cte2.Column2
FROM Cte1
    INNER JOIN Cte2 ON Cte2.Column1 = Cte1.Column1
ORDER BY Cte1.Column2
```

</td>
</tr>
<tr>
<td>
<ul>
<li>Replace Newlines</li>
<li>Lowercase</li>
<li>Space Count = 2</li>
</ul>
</td>
<td>

```sql
with Cte1 as (select Column1, Column2, Column3, Column4 from Table1),
  Cte2 as (select Column1, Column2 from Table2)
select distinct
  Cte1.*,
  Cte2.Column2
from Cte1
  inner join Cte2 on Cte2.Column1 = Cte1.Column1
order by Cte1.Column2
```

</td>
</tr>
<tr>
<td>
<ul>
<li>Replace Newlines</li>
<li>Char Count = 50</li>
</ul>
</td>
<td>

```sql
With Cte1 As (
        Select
            Column1,
            Column2,
            Column3,
            Column4
        From Table1
    ),
    Cte2 as (select Column1, Column2 FROM Table2)
Select Distinct
    Cte1.*,
    Cte2.Column2
From Cte1
    inner join Cte2 ON Cte2.Column1 = Cte1.Column1
Order By Cte1.Column2
```

</td>
</tr>
<tr>
<td>
<ul>
<li>Replace Newlines</li>
<li>Uppercase</li>
<li>Tabs</li>
<li>Char Count = 40</li>
</ul>
</td>
<td>

```sql
WITH Cte1 AS (
		SELECT
			Column1,
			Column2,
			Column3,
			Column4
		FROM Table1
	),
	Cte2 AS (
		SELECT
			Column1,
			Column2
		FROM Table2
	)
SELECT DISTINCT
	Cte1.*,
	Cte2.Column2
FROM Cte1
	INNER JOIN Cte2 ON Cte2.Column1 = Cte1.Column1
ORDER BY Cte1.Column2
```

</td>
</tr>
</tbody>
</table>
