import requests

s = requests.Session()

def visit_main_page(): #this visits the main page to get a JSESSIONID cookie
	global s
	s.get("https://parents.whrhs.org/genesis/sis/view?gohome=true")

def login_to_genesis(username, password): #this logs into genesis and makes the JSESSIONID valid
	global s
	query = {
		"j_username" : username,
		"j_password" : password
	}
	
	resp = s.post("https://parents.whrhs.org/genesis/sis/j_security_check", data=query)
	print("attempted login")

def apply_for_attendance(user_id): #this sends a POST request to the attendance section and sets you as present
	global s
	query = {
		str(user_id) : "on",
		"attendanceType" : "Present",
		"attendanceDate" : "",
		"attendanceReturnDate" : "",
		"fldReason" : ""
	}
	api_url = "https://parents.whrhs.org/genesis/parents?tab1=studentdata&tab2=attendance&tab3=notify&studentid={}&action=notifyOffice".format(user_id)
	s.post(api_url, data=query)
	print("attempted attendance post")

user_id = input("enter user id")
password = input("enter genesis password")
user_email = str(user_id) + "@whrhs-stu.org"

visit_main_page()
login_to_genesis(user_email, password)
apply_for_attendance(user_id)
print("done")