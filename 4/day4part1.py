from dateutil.parser import parse
from datetime import datetime, timedelta

guards = []
times = []
asleep_times = []
def parse_line(line):
    parts = line.split(' ')
    third_part = parts[2]
    date = parse(parts[0][1:])
    time = parts[1][3:5]
    if third_part.startswith('Guard'):
        if parts[1][0:2] == '23':
            guard_date = date + timedelta(days=1)
        else:
            guard_date = date
        guards.append({
            'id': parts[3][1:],
            'date': guard_date,
            'total_sleep_time': 0,
            'minutes': {}
        })
    elif third_part == 'falls':
        times.append({
            'date': date,
            'time': int(time),
        })
    elif third_part == "wakes":
        times.append({
            'date': date,
            'time': int(time),
        })

def count_minutes(minutes):
    total_minutes = 0
    for i in range(int(len(minutes)/2)):
        total_minutes += minutes[i*2+1] - minutes[i*2]
    return total_minutes

def get_guard(guards, id):    
    return next((x for x in guards if x['id'] == id), None)

def is_minute_in_intervals(minutes, minute):
    for i in range(int(len(minutes)/2)):
        if minute >= minutes[i*2] and minute < minutes[i*2+1]:
            return True
    return False

with open('input.txt') as f:
    for line in f:
        parse_line(line)

time_map = {}
for time in times:
    date = time['date']
    if date not in time_map:
        time_map[date] = {
            'guard': None,
            'times':[]
        }
    time.pop('date')
    time_map[date]['times'].append(time)
for k,v in time_map.items():
    v['times'].sort(key=lambda x: x['time'])
    # we can assume that times will always be divisible by 2 because guard has to wake up in order to leave
    v['times'] = list(o['time'] for o in v['times'])
    v['guard'] = [x for x in guards if x['date'] == k][0]['id']

guard_sleeping_minutes = []
guard_with_most_sleep = -1
max_sleep = 0
for k,v in time_map.items():
    day_sleep_time = count_minutes(v['times'])
    guard = get_guard(guards, v['guard'])
    guard['total_sleep_time'] += day_sleep_time
    if guard['total_sleep_time'] > max_sleep:
        max_sleep = guard['total_sleep_time']
        guard_with_most_sleep = guard['id']
    
# print(guards)
# print(guard_with_most_sleep)
# print(max_sleep)
# print max(node.y for node in path.nodes)
#array where index represents a minute ie. '15' minute 15 of an hour and value represents occurances
minutes_by_count = []
for minute in range(0, 60):
    minutes_by_count.append(0)
for k,v in time_map.items():
    if v['guard'] == guard_with_most_sleep:        
        for minute in range(0, 60):            
            if is_minute_in_intervals(v['times'], minute):
                minutes_by_count[minute] += 1

best_minute = minutes_by_count.index(max(minutes_by_count))
# print("Result first part:")
# print(int(guard_with_most_sleep)*best_minute)


# sleep by guard on minute
# for each guard, go through each minute of each date and count 

for guard in guards:
    for minute in range(60):
        guard['minutes'][minute] = 0


for k,v in time_map.items():
    for minute in range(60):
        if is_minute_in_intervals(v['times'], minute):
            get_guard(guards, v['guard'])['minutes'][minute] += 1

max_value = 0
max_guard_id = -1
max_minute = -1
for guard in guards:
    for k, v in guard['minutes'].items():
        # print(minute)
        if v > max_value:
            max_value = v
            max_guard_id = guard['id']
            max_minute = k

print(max_guard_id)
print(max_minute)
print(int(max_guard_id) * max_minute)