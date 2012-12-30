--- tracgantt/gantt.py.orig       2006-07-20 21:07:06.000000000 +0400
+++ tracgantt/gantt.py    2008-08-14 05:44:53.000000000 +0400
@@ -115,7 +115,7 @@
             show_opened = self.env.config.getbool('gantt-charts',
                     'show_opened', 'false')

-            tickets,dates,broken = self._tickets_for_report(db, report['query'])
+            tickets,dates,broken = self._tickets_for_report(db, report['query'], req.perm.username)
             tickets,dates = self._paginate_tickets(tickets, dates)

             req.hdf['gantt.tickets'] = tickets
@@ -163,7 +163,7 @@

         return {'id':id, 'title':title, 'query':query, 'description':description}

-    def _tickets_for_report(self, db, query):
+    def _tickets_for_report(self, db, query, username):
         """ Get a list of Ticket instances for the tickets in a report """

         tickets = []
@@ -172,7 +172,7 @@

         ## Get tickets for this report
         cursor = db.cursor()
-        cursor.execute(query)
+        cursor.execute(query.replace('$USER', "'%s'" % username))
         info = cursor.fetchall() or []
         cols = [s[0] for s in cursor.description or []]
         db.rollback()
@@ -344,7 +344,7 @@
                     "use_creation_date", "true")

             if use_cdate:
-                start = datetime.date.fromtimestamp(ticket.time_created)
+                start = datetime.date.fromordinal(ticket.time_created.toordinal())
             else:
                 raise ValueError, "Couldn't get start date"

@@ -370,8 +370,8 @@
                     % (str(ticket.id), str(start), str(due))

         # Finally the ticket itself's open and close dates
-        open = datetime.date.fromtimestamp(ticket.time_created)
-        changed = datetime.date.fromtimestamp(ticket.time_changed)
+        open = ticket.time_created.date()
+        changed = ticket.time_changed.date()

         return (start, due, open, changed)
