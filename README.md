Progect performance charactaristics when run on a standard public github actions runner(4 Threads, 16GB ram(though you can see we never come close to using that much), x64). I will add data for running on a machine with more avalible paralelism manually later.
<table>
  <thread>
    <tr>
      <th rowspan="2">Day</th>
      <th rowspan="2">Name</th>
      <th colspan="2">Input</th>
      <th rowspan="2">🔢🧵</th>
      <th colspan="4">Part 1</th>
      <th colspan="4">Part 2</th>
    </tr>
    <tr>
      <th>Time</th>
      <th>Size</th>
      <th>Time</th>
      <th colspan="2">Allocations</th>
      <th>Result</th>
      <th>Time</th>
      <th colspan="2">Allocations</th>
      <th>Result</th>
    </tr>
  </thread>
  <tbody id="results">
<tr>
<th>1</th>
<th>Historian Hysteria</th>
<td>34µs</td>
<td>13.7 KiB</td>
<th>❌</th>
<td>55µs</td>
<td>15.7 KiB</td><td>4</td>
<td>1223326</td>
<td>55µs</td>
<td>15.7 KiB</td><td>4</td>
<td>21070419</td>
</tr>
<tr>
<th>2</th>
<th>Red-Nosed Reports</th>
<td>62µs</td>
<td>18.9 KiB</td>
<th>❌</th>
<td>56µs</td>
<td>0 bytes</td><td>0</td>
<td>334</td>
<td>155µs</td>
<td>0 bytes</td><td>0</td>
<td>400</td>
</tr>
<tr>
<th>3</th>
<th>Mull It Over</th>
<td>54µs</td>
<td>16.8 KiB</td>
<th>❌</th>
<td>34µs</td>
<td>0 bytes</td><td>0</td>
<td>165225049</td>
<td>62µs</td>
<td>0 bytes</td><td>0</td>
<td>108830766</td>
</tr>
<tr>
<th>4</th>
<th>Ceres Search</th>
<td>53µs</td>
<td>19.3 KiB</td>
<th>❌</th>
<td>532µs</td>
<td>0 bytes</td><td>0</td>
<td>2514</td>
<td>199µs</td>
<td>0 bytes</td><td>0</td>
<td>1888</td>
</tr>
<tr>
<th>5</th>
<th>Print Queue</th>
<td>61µs</td>
<td>15.9 KiB</td>
<th>❌</th>
<td>286µs</td>
<td>50.8 KiB</td><td>2</td>
<td>5268</td>
<td>357µs</td>
<td>50.8 KiB</td><td>2</td>
<td>5799</td>
</tr>
<tr>
<th>6</th>
<th>Guard Gallivant</th>
<td>58µs</td>
<td>16.6 KiB</td>
<th>❌</th>
<td>63µs</td>
<td>16.6 KiB</td><td>1</td>
<td>5534</td>
<td>26ms</td>
<td>17.6 KiB</td><td>2</td>
<td>2262</td>
</tr>
<tr>
<th>"</th>
<th>"</th>
<th>"</th>
<th>"</th>
<th>✅</th>
<th></th>
<th></th>
<th></th>
<th></th>
<td>12ms</td>
<td>37.7 KiB</td><td>19</td>
<td>2262</td>
</tr>
<tr>
<th>7</th>
<th>Bridge Repair</th>
<td>42µs</td>
<td>24.2 KiB</td>
<th>❌</th>
<td>732µs</td>
<td>1.28 KiB</td><td>17906</td>
<td>882304362421</td>
<td>1ms</td>
<td>1.28 KiB</td><td>23940</td>
<td>145149066755184</td>
</tr>
<tr>
<th>"</th>
<th>"</th>
<th>"</th>
<th>"</th>
<th>✅</th>
<td>4ms</td>
<td>12.8 KiB</td><td>17944</td>
<td>882304362421</td>
<td>4ms</td>
<td>17.4 KiB</td><td>23978</td>
<td>145149066755184</td>
</tr>
</tbody>
</table>
